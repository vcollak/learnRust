fn main() {
    /*
    below fails because x gets out of scope and
    no longer lives after that scope. it's lifetime is
    shorter than r.
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
    */

    let string1 = String::from("abcd");
    let result;

    //without lifetimes this would not compile because
    //string2 is no longer valid after the inner scope and since we're
    //returning a reference to either x or y, both need to
    //be valid and live
    //lifetime syntax is about connecting the lifetimes of various parameters
    // and return values of functions. Once they’re connected, Rust has enough
    //information to allow memory-safe operations and disallow operations that
    // would create dangling pointers or otherwise violate memory safety
    {
        let string2 = "xyz";
        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is {}", result);
}

//to satisfy the need for both x and y to be live,
//we added lifetimes to the function with 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
