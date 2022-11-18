use std::io;

fn main() {
    //real numbers
    let x = 2.0; // f64 - implicitly defined as f64 thanks to assigned value
    let y: f32 = 3.0; // f32 - explicitly defined as f32

    //operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    //booleans
    let t = true;
    let f: bool = false; // with explicit type annotation

    //characters
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //tuples - can have different types
    //explicit types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //inferred types
    let tup = (500, 6.4, 1);

    //from tuple to individual variables (implicitly)
    let (x, y, z) = tup;

    //print a value y
    println!("The value of y is: {y}");
    //another way to print
    println!("The value of y is: {}", y);

    //get values form tuples directly
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //array (allocated in te stack)
    //cannot shrink or grow
    //infers the type and # of elements
    let a = [1, 2, 3, 4, 5];

    //infers the type and # of elements
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    //infers the type and # of elements
    //defined explicitly as string reference with 4 values
    let seasons: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];

    //type and then number of elements
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //initialize array with same values. We'll get 5 elements with
    //the value 3
    //same as let a = [3, 3, 3, 3, 3];
    let a = [3; 5];

    //get the value be index
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    //this will panic since we want index 100 and only have 5
    //let second = a[100];

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    //new string
    let mut index = String::new();

    //read from stdin and into index. notice it's a mutable reference
    //mutable because read_line will mutate the var. reference because we don't
    //want read_line to retain ownership
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    //shadow the index var into usize
    //trim spaces first
    //then parse the string into usize (number)
    //parse will return a Result enum
    //show an error is the Result enum was an err
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    //show the value from that array that matches the index
    //this code is a serious flaw. we're asking user to enter a value.
    //we then use that value as an index for an array value.
    //if the user enters a higher value than number of elements,
    //the code will panic:
    //  Please enter an array index.
    //  100
    //  thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100', src/main.rs:120:19
    //  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
