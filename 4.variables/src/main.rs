fn main() {
    ///////////////////////
    //Variable Mutability//
    ///////////////////////
    //Without mut the variable is immutable
    //Make sure it's mutable so we can assign 6 later
    println!("### Variables ###");

    //define mutable var x
    let mut x = 5;
    println!("the value of X is {x}");

    //mutate it
    x = 6;
    println!("the value of x is {x}");

    /////////////
    // Constants
    /////////////
    // 1. cannot be mutable
    // 2. must have a type
    // 3. cannot be set to an expression that changes at runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    //cannot change it - below will fail
    //const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 4;

    /////////////
    // Shadowing
    /////////////
    /*
     Shadowing is different from marking a variable as mut,
    because we’ll get a compile-time error if we accidentally
    try to reassign to this variable without using the let keyword.
    The other difference between mut and shadowing is that
    because we’re effectively creating a new variable when we
    use the let keyword again, we can change the type of the
    value but reuse the same name.
    */
    println!("### Shadowing ###");

    let x = 5;
    println!("The value of x is: {x}");

    //new variable us declared, but shadows the first one
    //the first one is no longer live
    let x = x + 1; //6
    println!("The value of x is: {x}");

    {
        let x = x * 2;
        //shadows so result is 12
        //but when the scope ends this variable no longer exists
        println!("The value of X in the inner scope is: {x}");
    }

    //x returns to 6 this is still a value from before the scope change
    println!("The value of x is: {x}");

    //shadowing to change the type
    //string type
    let spaces = "   ";

    //usize type
    let spaces = spaces.len();
}
