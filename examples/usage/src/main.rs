use std::{thread, time::Duration};

use mpac_rs::{BlockingReceive, BlockingSend};
// wip
fn main() {
    let (tx, rx) = mpac_rs::channel();

    thread::spawn(move || tx.send(5));
    thread::sleep(Duration::from_secs(1));
    let v = rx.recv().unwrap();
    println!("value was {}", v);
    assert!(v == 5);
}
