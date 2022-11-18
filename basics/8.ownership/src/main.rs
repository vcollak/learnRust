use std::mem::take;

fn main() {
    ////////////
    // Stack
    ////////////

    let mut x = 5;

    //copy x and place in y
    //it's a copy because X and Y are on the stack
    let y = x;

    x = 10;
    println!("{}", x); //10
    println!("{}", y); //5 because it's a copy of x (on stack) from before x changes

    ///////////
    // Heap
    ///////////

    let s1 = String::from("Hello Vlad");
    let s2 = s1;

    //s1 was moved to s2, which mean s1 is no longer available
    //bellow will not compile
    //println!("{}", s1);

    //unlike above, "clone" implements a deep copy - not just
    //moving a pointer as below.
    let s11 = String::from("Hello Amy");
    let s22 = s11.clone();
    println!("s11 is: {} and s22 is {}", s11, s22);

    //we can also create a reference to z11 from z12
    //this means that z12 simply points to the
    let z11 = String::from("Ahoj");
    let z12 = &z11;
    println!("z11 is {} and z12 is {}", z11, z12);

    //////////////////////////
    // Ownership and functions
    //////////////////////////
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    //cannot use s. s was moved to takes_ownership
    //and is no loner valid
    //println!("S is: {}", s);

    //
    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy (on the stack), so it's okay to still
                   // use x afterward

    //we can use X because it's on a stack
    //and implements copy. makes_copy
    //never takes ownership of anything
    println!("x is {}", x);

    //some_string in gives_ownership function is no longer owned by that
    //function. ownership moved to s111
    let s111 = gives_ownership();

    //ownership of s222 moved to takes_and_gives_ownership, but then returned
    let s222 = String::from("hello");
    let s333 = takes_and_gives_ownership(s222);
    println!("{}", s333);

    let s2222 = String::from("ref");
    pass_by_reference(&s2222);
    println!("s2222 is: {}", s2222);
}

fn pass_by_reference(my_str: &String) {
    println!("the pass_by_reference got this string: {}", my_str);
}

fn gives_ownership() -> String {
    let some_string = String::from("my string");
    some_string
}

fn takes_and_gives_ownership(some_string: String) -> String {
    some_string
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
