use std::sync::mpsc;
use std::thread::{self};
use std::time::Duration;

fn main() {
    //new channel
    //if we wanted to have multiple transcievers,
    //we can just clone the tx
    let (tx, rx) = mpsc::channel();

    //new thread
    thread::spawn(move || {
        //vector with multiple messages
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        //look for each message and send to
        //channel. sleep for 1 second
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //look through the channel
    //in this case rx is an iterator
    for received in rx {
        println!("Got: {}", received);
    }
}
