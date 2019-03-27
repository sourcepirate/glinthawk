//! The crate is equiped with the set of listerners
//! into the proc file system to monitor the changes
//! in resource consumption.
//! All the metrics are collected on periodic basic
//! inorder to do the time series analysis with
//! the resource consumption data.

extern crate chrono;
extern crate procfs;

use std::collections::HashMap;

pub trait DataPoint {
    /// Represents a datapoint with x and y fields.
    /// The trait acts as a base to collect all necessary
    /// fields to enable data collection.
    fn to_point(&self) -> (u64, HashMap<String, String>);
}
