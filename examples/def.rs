use std::ops::Deref;

#[derive(Debug)]
struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
struct Foo;

impl Foo {
    fn foo(&self) {
        println!("Foo");
    }
}
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = DerefExample { value: 'a' };
    println!("{:?}", x);
    assert_eq!('a', *x);
    let f = &&&Foo;
    f.foo();
}
