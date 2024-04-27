use std::{sync::mpsc, thread, time::Duration};
// this demo show how to use mpsc channel
fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hello world");
        println!("send data to rx: {}", val);
        let _ = tx.send(val);
        thread::sleep(Duration::from_secs(1));
        let val1 = String::from("hello world");
        let _ = tx.send(val1);
    });
    for result in rx {
        println!("rev data from tx {}", result);
    }
}
