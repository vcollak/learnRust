use std::{sync::mpsc, thread};

fn main() {
    //define the challen
    let (tx, rx) = mpsc::channel();

    //spawn a thread and send message to that channel
    thread::spawn(move || {
        //message
        let val = String::from("hi");

        //send message
        tx.send(val).unwrap();

        //cannot do this. the send function took ownership
        //of val
        //println!("Val is {}", val);
    });

    //get the message from the channel. recv()
    //will block until message is received
    let received = rx.recv().unwrap();
    println!("Got {}", received);
}
