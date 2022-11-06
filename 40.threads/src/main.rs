use std::thread;
use std::time::Duration;

fn main() {
    //regular function
    fn loopit() {
        for i in 1..10 {
            println!("number {} from loopit thread", i);

            //sleeps for 1 millisecond for each loop
            thread::sleep(Duration::from_millis(1));
        }
    }

    //spawn the function as a thread
    let loop_handle = thread::spawn(loopit);

    //join() waits for the associated thread to finish.
    //This function will return immediately if the associated thread has already finished.
    //join() returns a result so we're simply unwrapping to panic if it's an error
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

    //the closure can access "surrounding" vars like the "v" vector
    //closure, but we need to move the ownership using the "move" keyword
    //by default it borrows
    let v = vec![1, 2, 3];
    let vec_handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    //wait for thread to finish
    vec_handle.join().unwrap();

    //this continues main thread
    for i in 1..5 {
        println!("number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    //if handle.join().unwrap(); was here instead of above,
    //the threads would alternate
}
