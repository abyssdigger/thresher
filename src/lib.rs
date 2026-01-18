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

/*
// Явный перебор! Для базовой молотилки не нужен клиент, нужен только обработчик!
struct ThresherClient {
    throat: Option<mpsc::Sender<Message>>,
}

impl ThresherClient {
    pub fn send(
        &self,
        load: MsgPayload,
    ) -> Result<time::SystemTime, mpsc::SendError<MsgPayload>> {
        let res;
        if let Some(tx) = &self.throat {
            let now = time::SystemTime::now(); // вынести в наследников?
            match tx.send(Message {
                sent: now, // вынести в наследников?
                payload: load,
            }) {
                Ok(_) => return Ok(now),
                Err(send_error) => res = send_error.0.payload,
            }
        } else {
            res = load;
        }
        Err(mpsc::SendError(res))
    }
}
// ... in Thresher:
    pub fn new_sender(&self) -> ThresherClient {
        let throat;
        match &self.throat {
            Some(a) => throat = Some(a.clone()),
            None => throat = None,
        }
        ThresherClient { throat }
    }

*/
