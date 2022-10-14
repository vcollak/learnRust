use std::thread;
use std::time::Duration;

fn main() {
    //regular function
    fn loopit() {
        for i in 1..10 {
            println!("number {} from loopit thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    //spawn the function as a thread
    let loop_handle = thread::spawn(loopit);

    //wait till it finishes execution
    loop_handle.join().unwrap();

    //spawns a thread with a closure
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //waits till the thread finishes
    //if we don't do this, the the spawned handle thread may
    //exit prematurely when the main thread exists
    handle.join().unwrap();

    //we can pass the surroinding vars into a
    //closure, but we need to move the ownership
    //of v to the closure (by default it borrows)
    let v = vec![1, 2, 3];
    let vec_handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    vec_handle.join().unwrap();

    //the continues main thread
    for i in 1..5 {
        println!("number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    //if handle.join().unwrap(); was here instead of above,
    //the threads would alternate
}
