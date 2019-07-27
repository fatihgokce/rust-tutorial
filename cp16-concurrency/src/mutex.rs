use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    //16.3 shared state concurrency
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
       *num = 6;
    }

    println!("m = {:?}", m);
    //sharing mutex<T> between multiple threads
    //let counter = Mutex::new(0);
    //if we use Rc<T>,we get error because
    //Unfortunately, Rc<T> is not safe to share across threads.
    // When Rc<T> manages the reference count
    //So
    //Fortunately, Arc<T> is a type like Rc<T> that is safe to use in concurrent situations
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());


}