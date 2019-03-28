//! The file contains the implementation for
//! watching the memory consumption of a machine
//!

use super::{Metric, Shipper, Watcher};
use procfs::{meminfo, ProcResult};
use std::sync::mpsc::SendError;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

pub struct MemWatcher {
    tx: Arc<Mutex<Sender<Metric>>>,
}

impl Watcher for MemWatcher {
    fn watch(&self) -> ProcResult<Metric> {
        let info = meminfo();
        info.map(|x| Metric::Memory(x.mem_total, x.mem_free))
    }
}

impl Shipper for MemWatcher {
    fn send(&self, x: Metric) -> Result<(), SendError<Metric>> {
        self.tx.lock().unwrap().send(x)
    }
}
