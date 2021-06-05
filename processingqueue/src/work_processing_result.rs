pub enum WorkProcessingResult<T>
where
    T: Send,
{
    AddWork(Vec<T>),
    AddWorkAndCheckpoint(Vec<T>),
    Interrupt,
}
