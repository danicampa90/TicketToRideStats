use crossbeam::deque::{Injector, Stealer, Worker};
use std::iter;

pub enum Work {
    PrintDebug(i32),
}

pub struct Scheduler {
    global: Injector<Work>,
    stealers: Vec<Stealer<Work>>,
    nr_workers: usize,
}

impl Scheduler {
    pub fn new(nr_workers: usize) -> Scheduler {
        let mut stealers = vec![];
        //let mut locals = vec![];
        /*
                for _i in 0..nr_workers {
                    let wrk = Worker::new_lifo();
                    let stealer = wrk.stealer();
                    locals.push(wrk);
                    stealers.push(stealer);
                }
        */
        return Scheduler {
            global: Injector::new(),
            stealers: stealers,
            nr_workers: nr_workers,
        };
    }

    pub fn run<'a>(
        &'a mut self,
        function: &(dyn Fn(&Scheduler, Work) -> Vec<Work> + Send + Sync + 'a),
    ) {
        let mut join_handles = vec![];
        for _index in 0..self.nr_workers {
            // We know that this function will be only used in this function,
            // and all the threads will be done before this function is over,
            // so we transmute it to the 'static lifetime.
            // This is true also for &'static self and &'a self
            let func: &(dyn Fn(&Scheduler, Work) -> Vec<Work> + Send + Sync + 'static) =
                unsafe { std::mem::transmute(function) };

            let self_static: &'static Scheduler = unsafe { std::mem::transmute(&self) };

            let worker = Worker::new_fifo();
            self.stealers.push(worker.stealer());

            join_handles.push(std::thread::spawn(move || {
                self_static.single_thread(func, worker);
            }));
        }
        for join_handle in join_handles {
            join_handle.join().unwrap();
        }
    }

    pub fn single_thread(
        &'static self,
        function: &(dyn Fn(&Self, Work) -> Vec<Work> + Send + Sync + 'static),
        worker: Worker<Work>,
    ) {
        while let Some(x) = self.pop_task(&worker) {
            let w = function(self, x);
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

    pub fn is_done(&self) -> bool {
        false
    }

    fn pop_task(&self, local: &Worker<Work>) -> Option<Work> {
        // Pop a task from the local queue, if not empty.
        local.pop().or_else(|| {
            // Otherwise, we need to look for a task elsewhere.
            iter::repeat_with(|| {
                // Try stealing a batch of tasks from the global queue.
                self.global
                    .steal_batch_and_pop(local)
                    // Or try stealing a task from one of the other threads.
                    .or_else(|| self.stealers.iter().map(|s| s.steal()).collect())
            })
            // Loop while no task was stolen and any steal operation needs to be retried.
            .find(|s| !s.is_retry())
            // Extract the stolen task, if there is one.
            .and_then(|s| s.success())
        })
    }
}
