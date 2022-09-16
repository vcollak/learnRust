fn main() {
    /*
    Vectors allow you to store more than one value in a single data structure
    that puts all the values next to each other in memory.
    Vectors can only store values of the same type.
    They are useful when you have a list of items, such as the lines of text
    in a file or the prices of items in a shopping cart.
    */

    //define a vector using generics
    //this vector will hold i32 type
    let v: Vec<i32> = Vec::new();
    //----------------

    //define vector using a macro to assign initial values
    let vv = vec![1, 2, 3];
    //----------------

    let mut vvv = vec![1, 2, 3];
    vvv.push(4);
    vvv.push(5);
    //this will not compile since value needs to be i32
    //vvv.push(4.1);
    //----------------

    let vvvv = vec![1, 2, 3, 4, 5];

    //need to pass by reference since we're borrowing
    let third: &i32 = &vvvv[2];
    println!("The third element is {}", third);

    //this will panic with index out of bounds
    //let thirdPanic: &i32 = &vvvv[20];

    //a safer way is this. if we ask for an element out of range
    //we get a none because .get returns an Option
    let third: Option<&i32> = vvvv.get(20);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no 20th element"),
    }

    //iterate over the vector
    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{}", i);
    }

    //defines a mutable vector
    let mut v6 = vec![100, 32, 57];

    //iterate over a mutable vector
    //and multiply each element by 50
    for i in &mut v6 {
        //use * to dereference the reference so we can get out a value
        *i += 50;

        println!("{}", i);
    }

    //we can hold different types in vectors using enums
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    //Rust needs to know what types will be in the vector at compile time
    //so it knows exactly how much memory on the heap will be needed to
    //store each element.
    //If you don’t know the exhaustive set of types a program will get at
    //runtime to store in a vector, the enum technique won’t work.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
