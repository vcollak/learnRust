fn main() {
    print_hello();
    print_passed_number(2);
    print_number_and_label(10, 'h');

    //this is a statment. unline an expression, it does not return a value
    let _x = 10;

    //as a result, below will not compile
    //because statements do not return values
    //let num = (let y = 20);

    //below will compile because the {} create a new expression
    //that returns something
    let y = {
        let x = 3;

        /*
        Note that the x + 1 line doesn’t have a semicolon at the end,
        unlike most of the lines you’ve seen so far.
        Expressions do not include ending semicolons.
        If you add a semicolon to the end of an expression,
        you turn it into a statement, and it will then not return a value.
        Keep this in mind as you explore function return values and expressions next.
        */
        x + 1 //returns a value
    };
    println!("Y is {y}");

    let x = five();
    println!("The value of x is {x}");

    let num = plus_one(10);
    println!("The value os mum is: {num}");
}

fn print_hello() {
    println!("Hello");
}

fn print_passed_number(x: i32) {
    println!("Number {x}");
}

fn print_number_and_label(value: i32, unit_label: char) {
    println!("Number: {value} Label:{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
