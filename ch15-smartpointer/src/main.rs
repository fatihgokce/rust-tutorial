
use crate::List::{Cons, Nil};
use std::rc::Rc;
/*enum List {
    Cons(i32, Box<List>),
    Nil,
}*/
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
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
//    let b = Box::new(5);
//    let c:&i32=&5;
//    println!("b = {}", *b==*c);
//    let list = Cons(1,
//                    Box::new(Cons(2,
//                                  Box::new(Cons(3,
//                                                Box::new(Nil))))));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    //    hello(&(*m)[..]); Rust to handle these conversions for us automatically.
    //15.4 referenced counting
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    let a2 = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a2));
    let b2 = Cons(3, Rc::clone(&a2));
    println!("count after creating b = {}", Rc::strong_count(&a2));
    {
        let c = Cons(4, Rc::clone(&a2));
        println!("count after creating c = {}", Rc::strong_count(&a2));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a2));
}
