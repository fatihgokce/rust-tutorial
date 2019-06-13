enum List {
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
fn main() {
    let b = Box::new(5);
    let c:&i32=&5;
    println!("b = {}", *b==*c);
    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    //    hello(&(*m)[..]); Rust to handle these conversions for us automatically.

}
