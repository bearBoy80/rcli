use std::{thread, time::Duration};

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let thread = thread::spawn(|| {
        println!("hello world");
        thread::sleep(Duration::from_secs(1));
    });
    thread::spawn(move || println! {"{:?}",v});
    //wait thread finished
    thread.join().unwrap();
}
