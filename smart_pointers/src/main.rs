use std::mem::drop;
use std::ops::{Deref, Drop};

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

struct CustomSmartPoint {
    data: String,
}

impl Drop for CustomSmartPoint {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPoint {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPoint {
        data: String::from("other stuff"),
    };

    drop(c);

    println!("CustomSmartPointers created.");
}

fn hello(name: &str) {
    println!("Hello, {name}");
}
