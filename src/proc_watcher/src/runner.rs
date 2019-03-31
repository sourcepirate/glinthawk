use super::Metric;
use super::Watcher;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time;

pub struct Worker {
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn spawn(t: Arc<Mutex<Watcher>>, tx: mpsc::Sender<Metric>) -> Self {
        let rwatch = Arc::clone(&t);
        let transmitter = tx.clone();
        let handler = thread::spawn(move || loop {
            let guarded = rwatch.lock().unwrap();
            match guarded.watch() {
                Ok(m) => transmitter.send(m).unwrap(),
                Err(_) => warn!("Failed sending metric!!"),
            };
            thread::sleep(time::Duration::new(5, 0));
        });
        Worker {
            thread: Some(handler),
        }
    }
}

pub fn schedule<T: Watcher + 'static>(t: Vec<T>) -> (mpsc::Receiver<Metric>, Vec<Worker>) {
    let (tx, rx) = mpsc::channel::<Metric>();
    let workers = t
        .into_iter()
        .map(|x| Worker::spawn(Arc::new(Mutex::new(x)), tx.clone()))
        .collect();
    (rx, workers)
}
