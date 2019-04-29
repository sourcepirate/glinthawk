use super::Metric;
use super::Watcher;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time;

use super::load::LoadWatcher;
use super::memory::MemWatcher;
use super::network::NetworkWatcher;
use super::network::Protocol;
use super::process::ProcessWatcher;

pub fn run_watchers(interval: usize) -> (mpsc::Receiver<Metric>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel::<Metric>();
    let handle = thread::spawn(move || {
        let watchers: Vec<Box<Watcher>> = vec![
            Box::new(LoadWatcher),
            Box::new(NetworkWatcher {
                protocol: Protocol::TCP,
            }),
            Box::new(NetworkWatcher {
                protocol: Protocol::UDP,
            }),
            Box::new(ProcessWatcher),
            Box::new(MemWatcher),
        ];
        loop {
            for watcher in watchers.iter() {
                let watchresult = watcher.watch();
                match watchresult {
                    Ok(result) => tx.send(result).unwrap_or_default(),
                    Err(_) => info!("Unable to unwrap result"),
                };
            }
            thread::sleep(time::Duration::new(interval, 0));
        }
    });
    (rx, handle)
}
