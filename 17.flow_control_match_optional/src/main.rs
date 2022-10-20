//pass an Option and match
//for none or some. if we find some
//add 1 to the value, otherwise return None
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn some_function() {
    println!("some function executed");
}
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);

    let dice_roll = 3;
    match dice_roll {
        //matches a pattern and executes expressions. We could
        //also execute functions
        3 => println!("Three"),
        9 => println!("Nine"),
        2 => some_function(),
        //match has to be exhaustive. below is a catch all pattern
        //if pattern is not caught above, it will be other
        //here we are also using the other value
        //if we didn't use the value we could have used underscore
        //  _ => println!("other"),
        //  _ => ()
        other => println!("other {}", other),
    }
}
