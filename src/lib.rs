use std::{
    fmt::Debug,
    sync::mpsc,
    thread,
    time,
};

pub type MsgPayload = String;

#[derive(Debug)]
pub struct Message {
    pub sent: time::SystemTime, // Вынести в наследников
    pub payload: MsgPayload,
}

pub struct Thresher {
    throat: Option<mpsc::Sender<Message>>,
    worker: Option<thread::JoinHandle<()>>,
}

impl Thresher {
    pub fn new() -> Thresher {
        let (tx, rx): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();
        let handle = thread::spawn(|| {
            for received in rx {
                let dur = received.sent.duration_since(time::UNIX_EPOCH).unwrap();
                println!(
                    "@{}.{} got: `{}`",
                    dur.as_secs(),
                    dur.as_micros() % 1000000,
                    received.payload
                );
            }
        });
        Thresher {
            throat: Some(tx),
            worker: Some(handle),
        }
    }

    pub fn clone_tx(&self) -> Option<mpsc::Sender<Message>> {
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

impl Drop for Thresher {
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