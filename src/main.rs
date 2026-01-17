use std::{
    thread,
    time::{Duration, SystemTime},
};

fn main() {
    println!("Hello, world!");
    first_test();
}

fn first_test() {
    let _msg0 = thresher::Message {
        sent: SystemTime::now(),
        payload: String::from("Hello, world! from Thresher"),
    };
    thread::sleep(Duration::from_millis(1));
    let _msg1 = thresher::Message {
        sent: SystemTime::now(),
        payload: String::from("Good bye world! from Thresher"),
    };
    let tsr = thresher::Thresher::new();
    let tx0 = tsr.clone_tx().unwrap();
    _ = tx0.send(_msg0);
    {
        let tx1 = tsr.clone_tx().unwrap();
        _ = tx1.send(_msg1);
    }
    for _ in 1..=1000 {
        let _msg = thresher::Message {
            sent: SystemTime::now(),
            payload: String::from("Loop!"),
        };
        _ = tx0.send(_msg);
    }
    // let client = tsr.new_sender();
    // let _ = client.send(String::from("Hello, world! from CLIENT!!!"));
    // let _ = client.send(String::from("Good bye world! from CLIENT!!!"));
}
