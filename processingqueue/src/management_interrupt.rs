use std::sync::mpsc::{Receiver, SyncSender};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ManagementCommand {
    WaitCheckpoint,
    Continue,
    Interrupt,
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ManagementResponse {
    Ack,
    Done,
}

pub struct ManagementConnection {
    pub sender: SyncSender<ManagementResponse>,
    pub receiver: Receiver<ManagementCommand>,
}
