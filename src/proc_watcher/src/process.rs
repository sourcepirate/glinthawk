use super::{Metric, Watcher};
use chrono::{SecondsFormat, Utc};
use procfs::{all_processes, ProcResult};

pub struct ProcessWatcher;

impl Watcher for ProcessWatcher {
    fn watch(&self) -> ProcResult<Metric> {
        let current_time = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

        Ok(Metric::ProcessCount(
            all_processes().len() as u32,
            current_time,
        ))
    }
}
