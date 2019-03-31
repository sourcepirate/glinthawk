use super::{Metric, Watcher};
use procfs::{all_processes, ProcResult};

pub struct ProcessWatcher;

impl Watcher for ProcessWatcher {
    fn watch(&self) -> ProcResult<Metric> {
        Ok(Metric::ProcessCount(all_processes().len() as u32))
    }
}
