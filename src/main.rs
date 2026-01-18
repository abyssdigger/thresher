use core::time;
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use thresher::Thresher;

type MsgPayload = String;

#[derive(Debug)]
struct Message {
    pub sent: SystemTime, // Вынести в наследников
    pub payload: MsgPayload,
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

    let tsr = Thresher::<Message>::new::<i32>(
        128,
        time::Duration::new(5, 0),
        || 0,
        |context| context < &51,
        |received, context| {
            let dur = received.sent.duration_since(UNIX_EPOCH).unwrap();
            println!(
                "#{} ::: @{}.{} got: `{}`",
                *context,
                dur.as_secs(),
                dur.as_micros() % 1000000,
                received.payload
            );
            *context += 1
        },
    );
    let tx0 = tsr.clone_tx().unwrap();
    _ = tx0.send(_msg0);
    {
        let tx1 = tsr.clone_tx().unwrap();
        _ = tx1.send(_msg1);
    }
    for _ in 1..=200 {
        let _msg = Message {
            sent: SystemTime::now(),
            payload: String::from("Loop!"),
        };
        tx0.send(_msg).unwrap();
    }
    // let client = tsr.new_sender();
    // let _ = client.send(String::from("Hello, world! from CLIENT!!!"));
    // let _ = client.send(String::from("Good bye world! from CLIENT!!!"));
}
