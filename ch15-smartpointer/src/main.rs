
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
//15.5
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
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
//15.5

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}