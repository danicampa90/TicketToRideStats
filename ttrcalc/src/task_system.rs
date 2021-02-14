use crossbeam::atomic::AtomicCell;
use crossbeam::deque::{Injector, Stealer, Worker};
use crossbeam_utils::thread;
use std::iter;

pub trait WorkProcessor<T>
where
    T: Send,
{
    fn set_id(&mut self, id: usize);
    fn sleep(&self, oth: usize);
    fn resume(&self, oth: usize);
    fn done(&self);
    fn process(&self, w: T) -> Vec<T>;
}

pub struct Scheduler<T>
where
    T: Send,
{
    global: Injector<T>,
    waiting_workers: AtomicCell<usize>,
    nr_workers: usize,
}

impl<T> Scheduler<T>
where
    T: Send,
{
    pub fn new(nr_workers: usize) -> Scheduler<T> {
        return Scheduler {
            global: Injector::new(),
            waiting_workers: AtomicCell::from(0),
            nr_workers: nr_workers,
        };
    }

    pub fn run<TProc: WorkProcessor<T> + Clone + Send>(&mut self, processor: &TProc) {
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
            while let Some(x) = self.pop_task(&worker, stealers) {
                let w = function.process(x);
                self.push_tasks(w, &worker)
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
