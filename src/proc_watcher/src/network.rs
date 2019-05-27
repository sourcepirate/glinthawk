//! The file contains implementation for
//! network watcher which monitors tcp and udp connections.
//!
use super::{Metric, Watcher};
use chrono::{SecondsFormat, Utc};
use procfs::{tcp, tcp6, udp, udp6, ProcResult};

/// all protocol it watches for.
pub enum Protocol {
    TCP,
    TCP6,
    UDP,
    UDP6,
}

impl Protocol {
    pub fn get_metric(&self) -> ProcResult<Metric> {
        let current_time = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

        match &self {
            &Protocol::TCP => tcp().map(|x| Metric::TcpConn4(x.len() as u32, current_time)),
            &Protocol::TCP6 => tcp6().map(|x| Metric::TcpConn6(x.len() as u32, current_time)),
            &Protocol::UDP => udp().map(|x| Metric::UdpConn4(x.len() as u32, current_time)),
            &Protocol::UDP6 => udp6().map(|x| Metric::UdpConn6(x.len() as u32, current_time)),
        }
    }
}

pub struct NetworkWatcher {
    pub protocol: Protocol,
}

impl Watcher for NetworkWatcher {
    fn watch(&self) -> ProcResult<Metric> {
        self.protocol.get_metric()
    }
}
