use std::{sync::mpsc, thread, time};

pub trait ThreshEngine<T> {
    // where T: std::marker::Send, { <-- add if init has to be made in the parent thread
    fn new() -> Self;
    fn on_receive(&mut self, msg: &T);
    fn on_timeout(&mut self) {}
    fn on_disconnect(&mut self) {}
    fn is_workable(&self) -> bool {
        true
    }
}

pub struct Thresher<T> {
    throat: Option<mpsc::SyncSender<T>>,
    worker: Option<thread::JoinHandle<()>>,
}

impl<T: Send + 'static> Thresher<T> {
    pub fn start<U>(bound: usize, timeout: time::Duration) -> Thresher<T>
    where
        U: ThreshEngine<T> + 'static,
    {
        let (tx, rx) = mpsc::sync_channel::<T>(bound);
        let handle = thread::spawn(move || {
            let mut engine = U::new(); // <-- move it out if init has to be made in the parent thread
            while engine.is_workable() {
                let wait_for_msg = rx.recv_timeout(timeout);
                match wait_for_msg {
                    Err(e) => match e {
                        mpsc::RecvTimeoutError::Timeout => {
                            engine.on_timeout();
                            continue;
                        }
                        mpsc::RecvTimeoutError::Disconnected => break,
                    },
                    Ok(m) => {
                        engine.on_receive(&m);
                    }
                }
            }
            engine.on_disconnect();
        });
        Thresher {
            throat: Some(tx),
            worker: Some(handle),
        }
    }

    pub fn clone_tx(&self) -> Option<mpsc::SyncSender<T>> {
        match &self.throat {
            Some(a) => Some(a.clone()),
            None => None,
        }
    }
}

impl<T> Drop for Thresher<T> {
    /// Drop with thread join if exists
    fn drop(&mut self) {
        self.throat = None;
        if let Some(h) = self.worker.take() {
            let _ = h.join();
        }
    }
}

