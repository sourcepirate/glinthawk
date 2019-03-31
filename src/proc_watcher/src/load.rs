use super::{Metric, Watcher};
use procfs::{LoadAverage, ProcResult};

pub struct LoadWatcher;

impl Watcher for LoadWatcher {
    fn watch(&self) -> ProcResult<Metric> {
        LoadAverage::new().map(|x| Metric::Load(x.one))
    }
}
