//! The crate is equiped with the set of listerners
//! into the proc file system to monitor the changes
//! in resource consumption.
//! All the metrics are collected on periodic basic
//! inorder to do the time series analysis with
//! the resource consumption data.

extern crate procfs;

#[macro_use]
extern crate log;

use procfs::ProcResult;

pub mod load;
pub mod memory;
pub mod network;
pub mod process;
pub mod runner;

/// contains the assosiated mertics about the system
/// which includes network and ram information
pub enum Metric {
    /// Monitors the RAM consumption
    Memory(u64, u64),
    /// Monitors IPV4 connections
    TcpConn4(u32),
    /// Metrics for IPV6 tcp connections
    TcpConn6(u32),
    /// metrics for IPV4 udp4 connections
    UdpConn4(u32),
    /// metrics form IPV6 udp6 connections
    UdpConn6(u32),
    /// no of process running currently
    ProcessCount(u32),
    /// load average per minute
    Load(f32),
}

/// simple trait to watch for particular metric
pub trait Watcher: Send {
    /// watch for metric
    fn watch(&self) -> ProcResult<Metric>;
}

impl Metric {
    fn to_pair(&self) -> (String, f64) {
        match &self {
            &Metric::Memory(x, y) => (String::from("memory"), (*y / *x) as f64),
            &Metric::TcpConn4(x) => (String::from("tcp4"), *x as f64),
            &Metric::TcpConn6(x) => (String::from("tcp6"), *x as f64),
            &Metric::UdpConn4(x) => (String::from("udp4"), *x as f64),
            &Metric::UdpConn6(x) => (String::from("udp6"), *x as f64),
            &Metric::ProcessCount(x) => (String::from("proccess"), *x as f64),
            &Metric::Load(x) => (String::from("load"), *x as f64),
        }
    }
}
