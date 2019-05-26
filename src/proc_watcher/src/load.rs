use super::{Metric, Watcher};
use chrono::{SecondsFormat, Utc};
use procfs::{LoadAverage, ProcResult};

pub struct LoadWatcher;

impl Watcher for LoadWatcher {
    fn watch(&self) -> ProcResult<Metric> {
        let current_time = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        LoadAverage::new().map(|x| Metric::Load(x.one, current_time))
    }
}
