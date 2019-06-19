use std::thread;
use std::time::Duration;
use std::alloc::handle_alloc_error;
use std::sync::mpsc;
struct  Sn{
    str:String,
}
struct  Color(String);
fn main() {
    let v = vec![1, 2, 3];
    let handle=thread::spawn(move|| {
        println!("Here's a vector: {:?}", v);
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //println!("{:?}",v);
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    //16.2 Using Message Passing to Transfer Data Between Threads
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        println!("sending message");
        thread::sleep(Duration::from_millis(400));
        let s= Color(String::from("Mavi"));//Sn{str:String::from("deneme")};
        tx.send(s).unwrap();
    });
    //We’re using recv, short for receive, which will block the main thread’s execution
    // and wait until a value is sent down the channel
    let received = rx.recv().unwrap();
    println!("Got: {}", received.0);

    //Sending Multiple Values and Seeing the Receiver Waiting
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx2 {
        println!("Got2: {}", received);
    }

    //Creating Multiple Producers by Cloning the Transmitter
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("t2 more"),
            String::from("t2 messages"),
            String::from("t2 for"),
            String::from("t2 you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got3: {}", received);
    }
}