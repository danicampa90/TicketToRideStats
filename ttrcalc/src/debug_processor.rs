use crate::task_system::WorkProcessingResult;
use crate::task_system::WorkProcessor;

#[derive(Clone)]
pub struct DebugWorkProcessor {
    id: usize,
}

impl DebugWorkProcessor {
    pub fn new() -> DebugWorkProcessor {
        return DebugWorkProcessor { id: 0 };
    }
}

pub enum DebugWork {
    PrintDebug(i32),
}
impl WorkProcessor<DebugWork> for DebugWorkProcessor {
    fn process(self: &Self, w: DebugWork) -> WorkProcessingResult<DebugWork> {
        match w {
            DebugWork::PrintDebug(i) => {
                println!("[{}]: {}", self.id, i);
                return if i < 20 {
                    WorkProcessingResult::AddWork(vec![
                        DebugWork::PrintDebug(i + 1),
                        DebugWork::PrintDebug(i + 2),
                    ])
                } else {
                    WorkProcessingResult::AddWork(vec![])
                };
            }
        }
    }
    fn set_id(&mut self, id: usize) {
        self.id = id
    }
    fn done(&self) {
        println!("Thread {} is done", self.id);
    }
    fn sleep(&self, oth: usize) {
        println!(
            "Thread {} is going to sleep. There are {} other threads sleeping",
            self.id, oth
        );
    }
    fn resume(&self, oth: usize) {
        println!(
            "Thread {} is waking up. There are {} other threads sleeping.",
            self.id, oth
        );
    }
}
