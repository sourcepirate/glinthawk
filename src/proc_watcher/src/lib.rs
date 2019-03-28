//! The crate is equiped with the set of listerners
//! into the proc file system to monitor the changes
//! in resource consumption.
//! All the metrics are collected on periodic basic
//! inorder to do the time series analysis with
//! the resource consumption data.

extern crate chrono;
extern crate procfs;

use procfs::ProcResult;
use std::collections::HashMap;
use std::sync::mpsc::SendError;

pub mod memory;
pub mod network;

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
pub trait Watcher {
    /// watch for metric
    fn watch(&self) -> ProcResult<Metric>;
}

pub trait Shipper {
    fn send(&self, x: Metric) -> Result<(), SendError<Metric>>;
}
