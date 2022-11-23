fn main() {
    /*
    let favorite_color: Option<&str> = Some("red");
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    */

    let favorite_color: Option<&str> = None;
    let is_tuesday = true;
    let age: Result<u8, _> = "34".parse();

    /*
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
     */

    if let Some(color) = favorite_color {
        println!("color {}", color);
    } else if is_tuesday {
        println!("tuesday is green");
    } else if let Ok(age) = age {
        //creating a shadow variable age and using it in
        //the if block
        if age > 30 {
            println!("purple");
        } else {
            println!("orange");
        }
    } else {
        println!("blue");
    }

    //////
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    //while loop continues to pop from the stack
    //and assign the value in the Some(top) variable
    //we can then use top in the loop
    //The while loop continues running the code in
    //its block as long as pop returns Some.
    //When pop returns None, the loop stops.
    //We can use while let to pop every element off our stack.
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    ///////
    //loop through the vector
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    //////
    //Pattern matching literals
    let x = 3;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything else"),
    }

    //////
    let x = Some(5);
    //let x = None;
    let y = 10;

    match x {
        Some(50) => println!("got 50"),

        //because y is in a new scope, it's not the
        //same y as above. As a result, this will match
        //and Some value in this case Some(5)
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("default case, x = {:?}", x),
    }

    //this produces the original variables before the
    //match. The x and y in match were only in that
    //match scope
    println!("at the end: x = {:?}, y = {y}", x);

    ///////
    let x = 1;
    match x {
        //can use an or in the match by using "|"
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("something else"),
    }

    //////
    let x = 5;
    match x {
        //match a range
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'l';
    match x {
        //match a range of characters
        'a'..='j' => println!("a through j"),
        'k'..='z' => println!("k through z"),
        _ => println!("something else"),
    }

    //////
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    //we can also destructure a point
    //this pattern matched p into a Point and
    //assigned the a, b the values of the point
    let Point { x: a, y: b } = p;
    println!("{a}");
    println!("{b}");

    ///////
    //cannot shadow a struct like we can i32 in previous
    //exmaples where we could re-use x
    //lets call this PointA
    struct PointA {
        x: i32,
        y: i32,
    }

    let p = PointA { x: 0, y: 7 };

    //we can also use a shorthand without having to
    //create new vars a and b
    let PointA { x, y } = p;
    println!("{x}");
    println!("{y}");

    ///////
    let p = Point { x: 7, y: 0 };
    match p {
        //matches point where x is some value and y = 0
        Point { x, y: 0 } => println!("on x axis at {}", x),

        //matches point where y is some value and x = 0
        Point { x: 0, y } => println!("on y axis at {}", y),

        //matches point where x and y are non-zero values
        Point { x, y } => println!("neither on x axis or y axis ({}, {})", x, y),
    }

    ////////
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    //let msg = Message::Move { x: 10, y: 20 };

    match msg {
        //matches the varian of enum being Quit
        Message::Quit => {
            println!("quit variant. nothing to destructure");
        }

        //matches Move and destructures the values into x and y
        Message::Move { x, y } => {
            println!("move in x direction {} and in the y direction {}", x, y);
        }

        //matches Move and destructures the values into text
        Message::Write(text) => println!("Text message {}", text),

        //matches Move and destructures the values into r,g,b
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, blue {}", r, g, b)
        }
    }

    ///////
    // Pattern match nested enums and structs
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum MessageA {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = MessageA::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        //able to pattern match nested enums and structs
        MessageA::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, blue {}", r, g, b)
        }

        MessageA::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, value {}",
            h, s, v
        ),

        _ => (),
    }

    ///////
    //destructure structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {}", feet);
    println!("inches: {}", inches);
    println!("x: {}", x);
    println!("y: {}", y);

    ////////
    // ignoring entire value with _
    // wildcard pattern that will match any value but not bind to the value.
    // useful when implementing a trait that requires a certain signature, but
    // we don't need the value. avoids compiler warning of unused params
    fn foo(_: i32, y: i32) {
        println!("Code only uses y: {}", y);
    }

    foo(3, 4);

    ////////
    // ignoring part of the value with _

    //let mut setting_value = Some(5);
    //let new_setting_value = Some(10);

    let mut setting_value = None;
    let new_setting_value = Some(10);

    //if both the old and new setting value is of type Some,
    //we get an error message that we cannot override. We
    //actually don't care what the values are and can ignore with _
    //now, if one the values was say None, the (Some(_), Some(_))
    //would not match and default case will be matched (to override the setting_value)
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Cannot overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    ////////
    // ignore values in multiple places
    let numbers = (2, 4, 8, 16, 32);

    //match ignores some values in the tuple
    match numbers {
        (first, _, third, _, fifth) => {
            println!("some numbers First: {first}, Third: {third}, Fifth: {fifth}");
        }
    }

    ///////
    // Ignoring an Unused Variable by Starting Its Name with _
    //the compiler will only warn about unised_y because
    //_unused_x starts with a "_"
    //unlike a standalone "_" that gets completely ignored,
    //"_SOME_VAR" will be bound to this var

    let _unused_x = 5;
    let unused_y = 10;

    //we can still use the variable that starts with _, but
    //if we do should probably refactor to remove _
    println!("_unused_x: {} ", _unused_x);
    println!("unused_y: {} ", unused_y);

    ///////
    // Ignoring Remaining Parts of a Value with ..
    // The .. pattern ignores any parts of a value that we haven’t
    // explicitly matched in the rest of the pattern

    struct PointB {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = PointB { x: 0, y: 0, z: 0 };
    match origin {
        //arm of the march able to ignore the remaining values using ..
        PointB { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        //ignore everything but the first and last number
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    //////
    // more conditionals for match

    //let num = Some(4);
    let num = Some(5);
    match num {
        //matches some but it has to be even (x % 2 = 0)
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        //otherwise it's odd
        Some(x) => println!("The number {x} is odd"),
        //unless it's simply None
        None => (),
    }

    //matching multiple patterns
    let xp = 4;
    let yp = true;

    match xp {
        //if (xp is 4, 5 or 6) and (the yp is true), print yes
        4 | 5 | 6 if yp => println!("yes"),
        //otherwise, print no
        _ => println!("no"),
    }

    //// @ bindings
    // @ enables us to hold and compare a variable at the same time

    enum MessageB {
        Hello { id: i32 },
        Goodbye,
    }

    //let msg = MessageB::Hello { id: 5 };
    //let msg = MessageB::Hello { id: 10 };
    let msg = MessageB::Hello { id: 100 };

    match msg {
        //look for id being in the range of 3-7. then assign the id to
        //variable is_variable using variable @
        MessageB::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),

        //if the id is not in teh range of 3-7 then see if it's in the range of 10-12
        //don't worry about assigning the value
        //not able to print id becuase it has a pattern. would have to use
        // some_var @ 10..=12 to use the variable in print
        MessageB::Hello { id: 10..=12 } => println!("Found an id in another range"),

        //if it's still MessageB::Hello, but a different range, then print other
        //able to print id becuase it's not a pattern
        MessageB::Hello { id } => println!("Found some other id: {}", id),

        MessageB::Goodbye => println!("Goodbye"),
    }
}
