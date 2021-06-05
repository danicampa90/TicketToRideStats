use crate::work_processing_result::WorkProcessingResult;

pub trait InterruptHandler<TInterrupt, T>
where
    T: Send,
{
    fn handleInterrupt(&self, interrupt: TInterrupt, work: &Vec<T>);
}

pub enum SleepResult {
    SpinWait,
    ThreadWait,
    NoWait,
}

pub trait WorkProcessor<T>
where
    T: Send,
{
    fn set_id(&mut self, id: usize);
    fn no_work_available(&self) -> SleepResult;
    fn quit(&self);
    fn process(&self, w: T) -> WorkProcessingResult<T>;
}
