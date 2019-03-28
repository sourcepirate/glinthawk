//! The file contains implementation for
//! network watcher which monitors tcp and udp connections.
//!
use super::{Metric, Watcher};
use procfs::{tcp, tcp6, udp, udp6, ProcResult};
use std::sync::{Arc, Mutex};

/// all protocol it watches for.
pub enum Protocol {
    TCP,
    TCP6,
    UDP,
    UDP6,
}

impl Protocol {
    pub fn get_metric(&self) -> ProcResult<Metric> {
        match &self {
            &Protocol::TCP => tcp().map(|x| Metric::TcpConn4(x.len() as u32)),
            &Protocol::TCP6 => tcp6().map(|x| Metric::TcpConn6(x.len() as u32)),
            &Protocol::UDP => udp().map(|x| Metric::UdpConn4(x.len() as u32)),
            &Protocol::UDP6 => udp6().map(|x| Metric::UdpConn6(x.len() as u32)),
        }
    }
}

pub struct NetworkWatcher {
    version: u32,
    protocol: Protocol,
}

impl Watcher for NetworkWatcher {
    fn watch(&self) -> ProcResult<Metric> {
        self.protocol.get_metric()
    }
}
