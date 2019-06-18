use std::thread;
use std::time::Duration;
use std::alloc::handle_alloc_error;

fn main() {
    let v = vec![1, 2, 3];
    let handle=thread::spawn(move|| {
        println!("Here's a vector: {:?}", v);
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    println!("{:?}",v);
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

}