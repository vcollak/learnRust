fn main() {
    let s1 = String::from("hello");

    //instead of moving ownership, calculate_length
    //just borrows a reference to the string using &
    //why is the ownership not moving - because we
    //pass a reference that lives on the stack
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    //if we want to borrow we have to use a reference
    //but if we also want to mutable, we need to pass
    //a mutable reference. the string also needs to be
    //mutable
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);

    //if you have a mutable reference to a value,
    //you can have no other references to that value.
    //This code that attempts to create two mutable
    //references to s will fail
    let mut s3 = String::from("test");
    let r1 = &mut s3;

    //cannot have 2 mutable refereces.
    //bellow will fail
    //let r2 = &mut s3;
    //println!("{} {}", r1, r2);

    let mut s4 = String::from("hello");

    //ok to have multiple immutable references
    let r1 = &s4; // no problem
    let r2 = &s4; // no problem

    //cannot have two references to the same if
    //one is mutable and needs to be used.
    //let r3 = &mut s4; // BIG PROBLEM
    //println!("{}, {}, and {}", r1, r2, r3);

    let mut s5 = String::from("hello");

    let r1 = &s5; // no problem
    let r2 = &s5; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    //this works because r1 and r2 will no longer be used
    //after his so it's safe to mutate
    let r3 = &mut s5; // no problem
    println!("{}", r3);

    //but cannot do this because this would end-up
    //having mutable and immutable references pointing
    //to the same variable.
    //below will cause mutable ref above to have a compile error
    //The ability of the compiler to tell that a
    //reference is no longer being used at a point
    //before the end of the scope is called
    //Non-Lexical Lifetimes (NLL for short)
    //println!("{} and {}", r1, r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world");
}
