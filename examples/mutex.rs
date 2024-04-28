use std::{
    ops::Deref,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("{:?}", m);
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    for _ in 0..9 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.join().unwrap();
    }
    println!("result: {:?}", *counter.lock().unwrap());
    let x = 5;
    let y = MyBox::new(5);
    assert_eq!(x, *y)
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
