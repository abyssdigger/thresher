use core::time;
use std::{
    fmt::{self, Debug},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use thresher::{Thresher, ThreshEngine};

type MsgPayload = String;

#[derive(Debug)]
struct Message {
    pub sent: SystemTime, // Вынести в наследников
    pub payload: MsgPayload,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}'@{}", self.payload, self.sent.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO).as_millis())
    }
}

type SimpleCounter = u8;

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



fn main() {
    println!("Hello, world!");
    first_test();
}

fn first_test() {
    let _msg0 = Message {
        sent: SystemTime::now(),
        payload: String::from("Hello, world! from Thresher"),
    };
    thread::sleep(Duration::from_millis(1));
    let _msg1 = Message {
        sent: SystemTime::now(),
        payload: String::from("Good bye world! from Thresher"),
    };

    let tsr = Thresher::start::<SimpleCounter>(128, time::Duration::new(0, 5));

    let tx0 = tsr.clone_tx().unwrap();
    _ = tx0.send(_msg0);
    {
        let tx1 = tsr.clone_tx().unwrap();
        _ = tx1.send(_msg1);
    }
    for i in 1..=300 {
        let _msg = Message {
            sent: SystemTime::now(),
            payload: format!("Loop {i}"),
        };
        println!(">>>>>> Sending message {i}");
        if let Err(e) = tx0.send(_msg) {
            println!("Send error: {e}");
            break;
        };
    }
}

    // let client = tsr.new_sender();
    // let _ = client.send(String::from("Hello, world! from CLIENT!!!"));
    // let _ = client.send(String::from("Good bye world! from CLIENT!!!"));
