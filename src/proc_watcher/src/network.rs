//! The file contains implementation for
//! network watcher which monitors tcp and udp connections.
//!
use procfs::{tcp, tcp6, udp, udp6, ProcResult};
use std::sync::mpsc::SendError;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use super::Metric;

/// all protocol it watches for.
pub enum Protocol {
    TCP,
    TCP6,
    UDP,
    UDP6,
}

// impl Protocol {
//     pub fn get_metric(&self) -> ProcResult<Metric> {
//         match &self {
//             &Protocol::TCP =
//         }
//     }
// }

pub struct NetworkWatcher {
    tx: Arc<Mutex<Receiver<Metric>>>,
    version: u32,
    protocol: Protocol,
}
