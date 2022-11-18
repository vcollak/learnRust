use std::error::Error;
use std::fs;
use std::io::{self, Read};
use std::{fs::File, io::ErrorKind};

fn main() {
    //calling panic explicitly when there is an un-recoverable error
    //panic!("This will panic!");

    //causing panic inadvertently by accessing wrong index
    let v = vec![1, 2, 3];
    //v[99];

    //this does not actually return a file handle
    //instead, it returns a Result. This means that the File::open will not crash
    //when we try to open non-existent file

    //Below code does not gracefully recover so it's commented out:
    /*
    let greeting_file_result = File::open("hello.txt");
    let greeting_file_handle = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {:?}", error),
    }; */

    //a more elegant implementation that can recover if the file does not exist
    let file_result = File::open("hello.txt");
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        },
    };

    //Here is a way to automatically unwrap the result of OK. The file handle is returned.
    //if not, it will panic
    //let hello_file = File::open("hello_world.txt").unwrap();

    //we can also use expect to allow us to chose the panic message
    //let hello_file1 = File::open("hello_there.txt").expect("Expecting to open file there.txt");

    //handle reading of files in a separate function
    //the function returns a Result we can inspect
    let result = read_username_from_file();
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => panic!("{}", e),
    }

    //calling a shorter version that uses - ?
    let result = read_username_from_file_shorter();
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => panic!("{}", e),
    }

    //calling a shorter version that uses - ?
    let result = read_username_from_file_shorter_more();
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => panic!("{}", e),
    }

    //standard library has a way to handle this in a much shorter way
    let result = read_username_from_file_one_line();
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => panic!("{}", e),
    }
}

//returns either String (content from the file or en error)
fn read_username_from_file() -> Result<String, io::Error> {
    //get the result from opening a file
    let username_file_result = File::open("username.txt");

    //match for ok or error. if OK, code will proceed
    //if error we return with error
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    //file did exist so we got here
    let mut username = String::new();

    //let's read the file content and put in the username var
    //if succeeds, return the username String
    //otherwise, return an error
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    /*
    The ? placed after a Result value is defined to work in almost the same
    way as the match expressions we defined to handle the Result values earlier.
    If the value of the Result is an Ok, the value inside the Ok will get returned
    from this expression, and the program will continue. If the value is an Err,
    the Err will be returned from the whole function as if we had used the return
    keyword so the error value gets propagated to the calling code.
    */

    let mut username_file = File::open("username_shorter.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter_more() -> Result<String, io::Error> {
    //same as above but even more concise
    let mut username = String::new();

    File::open("username_shorter_more.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_one_line() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
