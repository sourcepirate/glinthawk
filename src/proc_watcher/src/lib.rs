//! The crate is equiped with the set of listerners
//! into the proc file system to monitor the changes
//! in resource consumption.
//! All the metrics are collected on periodic basic
//! inorder to do the time series analysis with
//! the resource consumption data.

extern crate chrono;
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
#[derive(Clone, Debug)]
pub enum Metric {
    /// Monitors the RAM consumption
    Memory(u64, u64, String),
    /// Monitors IPV4 connections
    TcpConn4(u32, String),
    /// Metrics for IPV6 tcp connections
    TcpConn6(u32, String),
    /// metrics for IPV4 udp4 connections
    UdpConn4(u32, String),
    /// metrics form IPV6 udp6 connections
    UdpConn6(u32, String),
    /// no of process running currently
    ProcessCount(u32, String),
    /// load average per minute
    Load(f32, String),
}

/// simple trait to watch for particular metric
pub trait Watcher: Send {
    /// watch for metric
    fn watch(&self) -> ProcResult<Metric>;
}

impl Metric {
    pub fn to_pair(&self) -> (String, f64) {
        match self {
            &Metric::Memory(x, y, _) => (
                String::from("memory"),
                (1.0 as f64 - (y as f64 / x as f64)) * 100.00 as f64,
            ),
            &Metric::TcpConn4(x, _) => (String::from("tcp4"), x as f64),
            &Metric::TcpConn6(x, _) => (String::from("tcp6"), x as f64),
            &Metric::UdpConn4(x, _) => (String::from("udp4"), x as f64),
            &Metric::UdpConn6(x, _) => (String::from("udp6"), x as f64),
            &Metric::ProcessCount(x, _) => (String::from("proccess"), x as f64),
            &Metric::Load(x, _) => (String::from("load"), x as f64),
        }
    }

    pub fn get_metric_name(&self) -> String {
        match self {
            &Metric::Memory(_, _, _) => String::from("MemoryConsumption"),
            &Metric::TcpConn4(_, _) => String::from("Tcp4Conn"),
            &Metric::TcpConn6(_, _) => String::from("Tcp6Conn"),
            &Metric::ProcessCount(_, _) => String::from("TotalProcess"),
            &Metric::UdpConn4(_, _) => String::from("Udp4Conn"),
            &Metric::UdpConn6(_, _) => String::from("Udp6Conn"),
            &Metric::Load(_, _) => String::from("LoadAvg"),
        }
    }

    pub fn get_metric_unit(&self) -> String {
        match self {
            &Metric::Memory(_, _, _) => String::from("Percent"),
            &Metric::TcpConn4(_, _) => String::from("Count"),
            &Metric::TcpConn6(_, _) => String::from("Count"),
            &Metric::ProcessCount(_, _) => String::from("Count"),
            &Metric::UdpConn4(_, _) => String::from("Count"),
            &Metric::UdpConn6(_, _) => String::from("Count"),
            &Metric::Load(_, _) => String::from("Percent"),
        }
    }

    pub fn get_time_stamp(&self) -> String {
        match self {
            &Metric::Memory(_, _, t) => t,
            &Metric::TcpConn4(_, t) => t,
            &Metric::TcpConn6(_, t) => t,
            &Metric::ProcessCount(_, t) => t,
            &Metric::UdpConn4(_, t) => t,
            &Metric::UdpConn6(_, t) => t,
            &Metric::Load(_, t) => t,
        }
    }
}
