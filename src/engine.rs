use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver},
    },
    thread, time,
};

pub trait ThreshEngine<T> {
    // where T: std::marker::Send, { <-- add if init has to be made in the parent thread
    fn init() -> Self;
    fn is_operable(&self) -> bool {
        true
    }
    fn on_receive(&mut self, msg: &T);
    fn on_timeout(&mut self) {}
    fn on_disconnect(&mut self) {}
}

pub struct Thresher<T: Send> {
    inloop: Arc<AtomicBool>,
    throat: Option<mpsc::SyncSender<T>>,
    worker: Option<thread::JoinHandle<()>>,
}

impl<Msg: Send + 'static> Thresher<Msg> {
    fn engine<Engine: ThreshEngine<Msg>>(
        rx: Receiver<Msg>,
        timeout: time::Duration,
        in_loop: Arc<AtomicBool>,
    ) {
        let mut engine = Engine::init(); // <-- move it out if init has to be made in the parent thread
        while in_loop.load(Ordering::Relaxed) && engine.is_operable() {
            let wait_for_msg: Result<Msg, mpsc::RecvTimeoutError> = rx.recv_timeout(timeout);
            match wait_for_msg {
                Err(e) => match e {
                    mpsc::RecvTimeoutError::Timeout => {
                        engine.on_timeout();
                        continue;
                    }
                    mpsc::RecvTimeoutError::Disconnected => {
                        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                        break;
                    }
                },
                Ok(m) => {
                    engine.on_receive(&m);
                }
            }
        }
        engine.on_disconnect();
    }

    pub fn start<Engine: ThreshEngine<Msg>>(
        bound: usize,
        timeout: time::Duration,
    ) -> Thresher<Msg> {
        let (tx, rx) = mpsc::sync_channel::<Msg>(bound);
        let inloop= Arc::new(AtomicBool::new(true));
        let inloop_clone = Arc::clone(&inloop);
        let handle: thread::JoinHandle<()> =
            thread::spawn(move || Self::engine::<Engine>(rx, timeout, inloop_clone));
        Thresher {
            inloop,
            throat: Some(tx),
            worker: Some(handle),
        }
    }

    pub fn clone_tx(&self) -> Option<mpsc::SyncSender<Msg>> {
        match &self.throat {
            Some(a) => Some(a.clone()),
            None => None,
        }
    }

    pub fn kill(self) {
    }

    pub fn halt(self) {
        self.inloop.store(false, Ordering::Relaxed);
    }
}

impl<T: Send> Drop for Thresher<T> {
    /// Drop with thread join if exists
    fn drop(&mut self) {
        self.throat = None;
        //self.inloop.store(false, Ordering::Relaxed);
        if let Some(h) = self.worker.take() {
            let _ = h.join();
        }
        println!("Dropped *************** ")
    }
}
