use crate::management_interrupt::ManagementCommand;
use crate::management_interrupt::ManagementConnection;
use crate::management_interrupt::ManagementResponse;
use crate::InterruptHandler;
use crate::WorkProcessingResult;
use crate::WorkProcessor;
use crossbeam::atomic::AtomicCell;
use crossbeam::deque::{Injector, Steal, Stealer, Worker};
use crossbeam::queue::ArrayQueue;
use crossbeam_utils::thread;
use std::iter;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::Arc;

pub struct Scheduler<T>
where
    T: Send,
{
    global: Injector<T>,
    waiting_workers: AtomicCell<usize>,
    nr_workers: usize,
    management_task: ArrayQueue<ManagementConnection>,
}

impl<T> Scheduler<T>
where
    T: Send,
{
    pub fn new(nr_workers: usize) -> Scheduler<T> {
        return Scheduler::<T> {
            global: Injector::new(),
            waiting_workers: AtomicCell::from(0),
            nr_workers: nr_workers,
            management_task: ArrayQueue::new(nr_workers),
        };
    }

    pub fn run<
        TInterrupt,
        TProc: WorkProcessor<T> + Clone + Send,
        TManagementProcessor: InterruptHandler<TInterrupt, T> + Send + Sync,
    >(
        &mut self,
        processor: &TProc,
        management_processor: TManagementProcessor,
    ) {
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
                    self.single_thread(proc, worker, &stealers, &management_processor);
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
        //self.checkpointer.checkpoint(&all_work);
unimplemented!("checkpointer above should be renamed and re-enabled")

        // push back all the work items into the global queue.
        for item in all_work.into_iter() {
            self.global.push(item);
        }
    }

    pub fn single_thread<
        TInterrupt,
        TProc: WorkProcessor<T>,
        TManagementProcessor: InterruptHandler<TInterrupt, T>,
    >(
        &self,
        function: TProc,
        worker: Worker<T>,
        stealers: &Vec<Stealer<T>>,
        management_processor: &TManagementProcessor,
    ) {
        // if we are done, then exit
        let backoff = crossbeam::utils::Backoff::new();
        loop {
            // process as much work as possible before sleeping
            while let Some(task) = self.pop_task(&worker, stealers) {
                // handle management commands
                if let Ok((mgmt_cmd, mgmt_conn)) = self.check_fetch_management_command() {
                    match mgmt_cmd {
                        ManagementCommand::Interrupt => {
                            self.waiting_workers.fetch_add(1);
                            function.quit();
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

                // call the processor
                let result = function.process(task);

                // process the result of the processing
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
                        function.quit();
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

            // mark as waiting, wait a bit and then wakeup. other threads might have added some stuff to process.
            let other_waiting_threads = self.waiting_workers.fetch_add(1);
            let action = function.no_work_available();
            // TODO: do something with action.
            unimplemented!("action now handled");
            backoff.snooze();
            let other_waiting_threads = self.waiting_workers.fetch_sub(1);
            // if everybody was waiting then we are done. Exit
            if other_waiting_threads == self.nr_workers {
                self.waiting_workers.fetch_add(1);
                function.quit();
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
