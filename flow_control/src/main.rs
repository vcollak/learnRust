fn main() {
    let number = 3;

    ////////////////////
    // Conditionals
    ////////////////////

    //simple if statement
    if number > 5 {
        println!("more than 5");
    } else if number == 5 {
        println!("equal 5");
    } else {
        println!("less than 5");
    }

    // not equal
    if number != 0 {
        println!("not equal to zero")
    }

    //assign the result of an IF expression into
    //a variable
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Number: {}", number);

    //this fails because the expression
    //does not return the same type
    //let number = if condition { 5 } else { "six" };

    ////////////////////
    // Loops
    ////////////////////
    //Rust has three kinds of loops: loop, while, and for.

    //loop - jut loops forever
    //Rust also provides a way to break out of a loop using code.
    //You can place the break or keyword

    /*
    loop {
        println!("Loop...");
    }
    */

    //returning a value from a loop using break
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result is {result}");

    //loop labels
    println!("## Loop Labels ##");
    let mut count = 0;
    'counting_up: loop {
        println!("count {count}");
        let mut remaining = 10;

        loop {
            println!("remaining {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    //while
    println!("## While ##");
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("Liftoff");

    //another while, but prone to mistakes such as
    //setting index to a wrong value
    //if index is set to higher number than the
    //number of elements, it will panic
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("value: {}", a[index]);
        index += 1;
    }

    //safer approach - just iterate over
    //the elements
    let b = [10, 20, 30, 40, 50];
    for element in b {
        println!("value is {element}");
    }

    //a better way to do countdown
    //this uses a range 1-4, but reverses it so it's 3,2,1
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("FOR liftoff");
}
