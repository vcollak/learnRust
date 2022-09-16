use std::io;

fn main() {
    //integers
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

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

    //tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

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

    //type and then number of elements
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //initialize array with same values
    let a = [3; 5]; //same as let a = [3, 3, 3, 3, 3];

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    //runtime errors with error overflow
    /*
    Please enter an array index.
    100
    thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100', src/main.rs:88:19
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
