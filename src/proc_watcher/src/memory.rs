//! The file contains the implementation for
//! watching the memory consumption of a machine
//!

use super::{Metric, Shipper, Watcher};
use procfs::{meminfo, ProcResult};

pub struct MemWatcher;

impl Watcher for MemWatcher {
    fn watch(&self) -> ProcResult<Metric> {
        let info = meminfo();
        info.map(|x| Metric::Memory(x.mem_total, x.mem_free))
    }
}
