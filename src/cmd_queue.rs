use std::{fmt, time::{Duration, SystemTime, UNIX_EPOCH}};

use crate::engine::ThreshEngine;

type MsgPayload = String;

#[derive(Debug)]
pub struct Message {
    pub sent: SystemTime, // Вынести в наследников
    pub payload: MsgPayload,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}'@{}", self.payload, self.sent.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO).as_millis())
    }
}

pub type SimpleCounter = u8;

impl ThreshEngine<Message> for SimpleCounter 
{
    fn new() -> Self {
        0
    }

    fn on_receive(&mut self, msg: &Message) {
        println!("{:#04X}->{}", *self, msg);
        *self = self.wrapping_add(1);
    }

    fn on_timeout(&mut self) {
        println!("##### TIMEOUT!");
    }

    fn on_disconnect(&mut self) {
        println!("##### Disconnected from Thresher!");
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
