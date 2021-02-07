use crossbeam::deque::{Injector, Stealer, Worker};
use crossbeam_utils::thread;
use std::iter;
use std::sync::Arc;

pub enum Work {
    PrintDebug(i32),
}

pub trait WorkProcessor {
    fn process(self: &Self, w: Work) -> Vec<Work>;
}

pub struct Scheduler {
    global: Injector<Work>,
    nr_workers: usize,
}

impl Scheduler {
    pub fn new(nr_workers: usize) -> Scheduler {
        return Scheduler {
            global: Injector::new(),
            nr_workers: nr_workers,
        };
    }

    pub fn run<T: WorkProcessor + Clone + Send>(&mut self, processor: &T) {
        let mut stealers = vec![];
        let mut workers = vec![];
        for _index in 0..self.nr_workers {
            let worker = Worker::new_fifo();
            stealers.push(worker.stealer());
            workers.push(worker);
        }
        thread::scope(|s| {
            for worker in workers.into_iter() {
                let proc = processor.clone();

                s.spawn(|_| {
                    self.single_thread(proc, worker, &stealers);
                });
            }
        })
        .unwrap();
    }

    pub fn single_thread<T: WorkProcessor>(
        &self,
        function: T,
        worker: Worker<Work>,
        stealers: &Vec<Stealer<Work>>,
    ) {
        while let Some(x) = self.pop_task(&worker, stealers) {
            let w = function.process(x);
            self.push_tasks(w, &worker)
        }
    }

    pub fn push_tasks(&self, work: Vec<Work>, worker: &Worker<Work>) {
        if worker.len() < 50 {
            for w in work.into_iter() {
                worker.push(w);
            }
        } else {
            for w in work.into_iter() {
                self.global.push(w);
            }
        }
    }

    pub fn push_task(&self, w: Work) {
        self.global.push(w);
    }

    pub fn is_done(&self) -> bool {
        false
    }

    fn pop_task(&self, local: &Worker<Work>, stealers: &Vec<Stealer<Work>>) -> Option<Work> {
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
