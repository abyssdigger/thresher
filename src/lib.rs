use std::{sync::mpsc, thread, time};

pub type MsgPayload = String;

#[derive(Debug)]
pub struct Message {
    pub sent: time::SystemTime, // Вынести в наследников
    pub payload: MsgPayload,
}

pub struct Thresher<MSG> {
    throat: Option<mpsc::SyncSender<MSG>>,
    worker: Option<thread::JoinHandle<()>>,
}

impl<MSG: 'static + Send> Thresher<MSG> {
    pub fn new<CTX: 'static>(
        bound: usize,
        timeout: time::Duration,
        init: fn() -> CTX,
        before: fn(&CTX) -> bool,
        on_msg: fn(MSG, &CTX) -> CTX,
    ) -> Thresher<MSG> {
        let (tx, rx) = mpsc::sync_channel::<MSG>(bound);
        let handle = thread::spawn(move || {
            let mut context = init();
            while before(&context) {
                //for received in rx {
                let wait_for_msg = rx.recv_timeout(timeout);
                match wait_for_msg {
                    Err(e) => match e {
                        mpsc::RecvTimeoutError::Timeout => continue,
                        mpsc::RecvTimeoutError::Disconnected => break,
                    },
                    Ok(m) => {
                        context = on_msg(m, &context);
                    }
                }
            }
        });
        Thresher {
            throat: Some(tx),
            worker: Some(handle),
        }
    }

    pub fn clone_tx(&self) -> Option<mpsc::SyncSender<MSG>> {
        match &self.throat {
            Some(a) => Some(a.clone()),
            None => None,
        }
    }

    /*pub fn new_sender(&self) -> ThresherClient {
        let throat;
        match &self.throat {
            Some(a) => throat = Some(a.clone()),
            None => throat = None,
        }
        ThresherClient { throat }
    }*/
}

impl<MSG> Drop for Thresher<MSG> {
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
*/
