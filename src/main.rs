use core::time;
use random_string::generate;
use std::{
    thread,
    time::{Duration, SystemTime},
};
use thresher::{
    cmd_queue::{CmdMessage, ShortCounter},
    engine::Thresher,
};

fn main() {
    println!("Hello, world!");
    first_test();
}

fn first_test() {
    let _msg0 = CmdMessage {
        sent: SystemTime::now(),
        payload: String::from("Hello, world! from Thresher"),
    };
    thread::sleep(Duration::from_millis(1));
    let _msg1 = CmdMessage {
        sent: SystemTime::now(),
        payload: String::from("Good bye world! from Thresher"),
    };

    let trshr = Thresher::start::<ShortCounter>(128, time::Duration::new(0, 100 * 1000 * 1000));

    {
        let tx0 = trshr.clone_tx().unwrap();
        _ = tx0.send(_msg0);
        {
            let tx1 = trshr.clone_tx().unwrap();
            _ = tx1.send(_msg1);
        }
        for i in 1..=300 {
            let _msg = CmdMessage {
                sent: SystemTime::now(),
                payload: format!("Loop {i} + {}", generate(6, "1234567890")),
            };
            println!(">>>>>> Sending message {i}");
            if let Err(e) = tx0.send(_msg) {
                println!("Send error: {e}");
                break;
            };
        }
    }
    //trshr.kill();
    trshr.halt();
    thread::sleep(Duration::from_secs(1));
    println!("========== FINITA =============");
}

// let client = tsr.new_sender();
// let _ = client.send(String::from("Hello, world! from CLIENT!!!"));
// let _ = client.send(String::from("Good bye world! from CLIENT!!!"));
