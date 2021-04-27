use crossbeam::atomic::AtomicCell;
use crossbeam::deque::{Injector, Steal, Stealer, Worker};
use crossbeam::queue::ArrayQueue;
use crossbeam_utils::thread;
use std::iter;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

pub trait Checkpointer<T>
where
    T: Send,
{
    fn checkpoint(&self, work: &Vec<T>);
}

pub enum WorkProcessingResult<T>
where
    T: Send,
{
    AddWork(Vec<T>),
    AddWorkAndCheckpoint(Vec<T>),
    Interrupt,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum ManagementCommand {
    WaitCheckpoint,
    Continue,
    Interrupt,
}
#[derive(Clone, PartialEq, Eq, Debug)]
enum ManagementResponse {
    Ack,
    Done,
}

struct ManagementConnection {
    sender: SyncSender<ManagementResponse>,
    receiver: Receiver<ManagementCommand>,
}

pub trait WorkProcessor<T>
where
    T: Send,
{
    fn set_id(&mut self, id: usize);
    fn sleep(&self, oth: usize);
    fn resume(&self, oth: usize);
    fn done(&self);
    fn process(&self, w: T) -> WorkProcessingResult<T>;
}

pub struct Scheduler<T, TCheckpointer>
where
    T: Send,
    TCheckpointer: Checkpointer<T> + Sync,
{
    checkpointer: TCheckpointer,
    global: Injector<T>,
    waiting_workers: AtomicCell<usize>,
    nr_workers: usize,
    management_task: ArrayQueue<ManagementConnection>,
}

impl<T, TCheckpointer> Scheduler<T, TCheckpointer>
where
    T: Send,
    TCheckpointer: Checkpointer<T> + Sync,
{
    pub fn new(nr_workers: usize, checkpointer: TCheckpointer) -> Scheduler<T, TCheckpointer> {
        return Scheduler {
            global: Injector::new(),
            waiting_workers: AtomicCell::from(0),
            nr_workers: nr_workers,
            management_task: ArrayQueue::new(nr_workers),
            checkpointer: checkpointer,
        };
    }

    pub fn run<TProc: WorkProcessor<T> + Clone + Send>(&mut self, processor: &TProc) {
        // reset state
        while self.management_task.pop().is_some() {}

        // setup the workers / stealers queues
        let mut stealers = vec![];
        let mut workers = vec![];
        for _index in 0..self.nr_workers {
            let worker = Worker::new_lifo();
            stealers.push(worker.stealer());
            workers.push(worker);
        }
        let mut worker_id = 0;
        thread::scope(|s| {
            for worker in workers.into_iter() {
                let mut proc = processor.clone();
                proc.set_id(worker_id);

                s.spawn(|_| {
                    self.single_thread(proc, worker, &stealers);
                });
                worker_id += 1;
            }
        })
        .unwrap();
    }

    fn send_management_task_to_all_others(
        &self,
        task: ManagementCommand,
    ) -> Vec<(SyncSender<ManagementCommand>, Receiver<ManagementResponse>)> {
        let mut commChannels = vec![];
        // starts from 1 because the current thread is excluded
        for _ in 1..self.nr_workers {
            let (sender_mgmt_to_thread, recv_mgmt_to_thread) = sync_channel(1);
            let (sender_thread_to_mgmt, recv_thread_to_mgmt) = sync_channel(1);
            let channel = ManagementConnection {
                sender: sender_thread_to_mgmt,
                receiver: recv_mgmt_to_thread,
            };
            if self.management_task.push(channel).is_err() {
                panic!("Can't add anymore to the management task. Somebody else is sending management messages too");
            }
            commChannels.push((sender_mgmt_to_thread, recv_thread_to_mgmt));
        }

        // start sending messages to all other threads
        for (sender, _receiver) in &commChannels {
            sender.send(task.clone()).unwrap();
        }
        // wait acks for them having received the message.
        for (_sender, receiver) in &commChannels {
            assert!(receiver.recv().unwrap() == ManagementResponse::Ack);
        }
        return commChannels;
    }

    fn check_fetch_management_command(
        &self,
    ) -> Result<(ManagementCommand, ManagementConnection), ()> {
        let mgmt_task = self.management_task.pop();
        match mgmt_task {
            None => Err(()),
            Some(connection) => {
                let command = connection.receiver.recv().unwrap();
                connection.sender.send(ManagementResponse::Ack).unwrap();
                Ok((command, connection))
            }
        }
    }

    fn receive_from_all(
        conns: &Vec<(SyncSender<ManagementCommand>, Receiver<ManagementResponse>)>,
    ) -> Result<Vec<ManagementResponse>, ()> {
        let mut result = vec![];
        for (sender, recv) in conns {
            result.push(recv.recv().unwrap());
        }
        Ok(result)
    }
    fn send_to_all(
        conns: &Vec<(SyncSender<ManagementCommand>, Receiver<ManagementResponse>)>,
        msg: ManagementCommand,
    ) -> Result<(), ()> {
        for (sender, recv) in conns {
            sender.send(msg.clone()).unwrap();
        }
        // wait acks for them having received the message.
        for (_sender, receiver) in conns {
            assert!(receiver.recv().unwrap() == ManagementResponse::Ack);
        }
        Ok(())
    }

    fn save_checkpoint(&self, stealers: &Vec<Stealer<T>>) {
        // we can only steal tasks into workers, so let's do that and then convert all the work into a vector.
        let tmp_worker = Worker::new_lifo();
        for stealer in stealers {
            while !stealer.steal_batch(&tmp_worker).is_empty() {}
        }
        while !self.global.steal_batch(&tmp_worker).is_empty() {}

        // collect work into an array
        let mut all_work = vec![];
        while let Some(work) = tmp_worker.pop() {
            all_work.push(work);
        }
        // checkpoint
        self.checkpointer.checkpoint(&all_work);

        // push back all the work items into the global queue.
        for item in all_work.into_iter() {
            self.global.push(item);
        }
    }

    pub fn single_thread<TProc: WorkProcessor<T>>(
        &self,
        function: TProc,
        worker: Worker<T>,
        stealers: &Vec<Stealer<T>>,
    ) {
        // if we are done, then exit
        let backoff = crossbeam::utils::Backoff::new();
        loop {
            // process as much work as possible
            while let Some(task) = self.pop_task(&worker, stealers) {
                if let Ok((mgmt_cmd, mgmt_conn)) = self.check_fetch_management_command() {
                    match mgmt_cmd {
                        ManagementCommand::Interrupt => {
                            self.waiting_workers.fetch_add(1);
                            function.done();
                            mgmt_conn.sender.send(ManagementResponse::Done).unwrap();
                            return;
                        }
                        ManagementCommand::WaitCheckpoint => {
                            assert!(
                                mgmt_conn.receiver.recv().unwrap() == ManagementCommand::Continue
                            );
                            mgmt_conn.sender.send(ManagementResponse::Ack).unwrap();
                            mgmt_conn.sender.send(ManagementResponse::Done).unwrap();
                        }
                        _ => panic!("Unrecognized command"),
                    }
                }
                let result = function.process(task);
                match result {
                    WorkProcessingResult::AddWork(work) => self.push_tasks(work, &worker),
                    WorkProcessingResult::Interrupt => {
                        print!("Requesting stop");
                        let others_conns =
                            self.send_management_task_to_all_others(ManagementCommand::Interrupt);
                        // now we have every other thread waiting for us.ManagementResponse
                        let all_done = Self::receive_from_all(&others_conns)
                            .unwrap()
                            .into_iter()
                            .all(|val| val == ManagementResponse::Done);
                        assert!(all_done);
                        self.waiting_workers.fetch_add(1);
                        function.done();
                        print!("Stop successful");
                        return;
                    }
                    WorkProcessingResult::AddWorkAndCheckpoint(work) => {
                        self.push_tasks(work, &worker);
                        print!("Requesting pause for checkpointing.");
                        let others_conns = self
                            .send_management_task_to_all_others(ManagementCommand::WaitCheckpoint);

                        self.save_checkpoint(&stealers);
                        Self::send_to_all(&others_conns, ManagementCommand::Continue).unwrap();
                        let all_done = Self::receive_from_all(&others_conns)
                            .unwrap()
                            .into_iter()
                            .all(|val| val == ManagementResponse::Done);
                        assert!(all_done);
                        println!("Resuming after checkpointing.");
                    }
                }
            }

            // mark as waiting, wait a bit and then wakeup
            let other_waiting_threads = self.waiting_workers.fetch_add(1);
            function.sleep(other_waiting_threads);
            backoff.snooze();
            let other_waiting_threads = self.waiting_workers.fetch_sub(1);
            function.resume(other_waiting_threads);
            // if everybody was waiting then we are done. Exit
            if other_waiting_threads == self.nr_workers {
                self.waiting_workers.fetch_add(1);
                function.done();
                return;
            }
        }
    }

    pub fn push_tasks(&self, work: Vec<T>, worker: &Worker<T>) {
        if worker.len() < 1000 {
            for w in work.into_iter() {
                worker.push(w);
            }
        } else {
            for w in work.into_iter() {
                self.global.push(w);
            }
        }
    }

    pub fn push_task(&self, w: T) {
        self.global.push(w);
    }

    pub fn is_done(&self) -> bool {
        false
    }

    fn pop_task(&self, local: &Worker<T>, stealers: &Vec<Stealer<T>>) -> Option<T> {
        // Pop a task from the local queue, if not empty.
        local.pop().or_else(|| {
            // Otherwise, we need to look for a task elsewhere.
            iter::repeat_with(|| {
                // Try stealing a batch of tasks from the global queue.
                self.global
                    .steal_batch_and_pop(local)
                    // Or try stealing a task from one of the other threads.
                    .or_else(|| stealers.iter().map(|s| s.steal()).collect())
            })
            // Loop while no task was stolen and any steal operation needs to be retried.
            .find(|s| !s.is_retry())
            // Extract the stolen task, if there is one.
            .and_then(|s| s.success())
        })
    }
}
