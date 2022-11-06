use std::fmt::format;

fn main() {
    //////////////////////
    // Creating a string
    //////////////////////

    //new instance of a mutable string
    let mut s = String::new();

    //str literal converted to String type
    let data = "string literal";
    let s1 = data.to_string();

    //can also convert directly on the literal
    let s2 = "Some String".to_string();

    //another way to create a String from string literal
    let s3 = String::from("same thing");

    //can do unicode
    let hello = String::from("Dobrý den");

    //////////////////////
    // Updating a string
    //////////////////////

    let mut lol = String::from("lo");

    //push a character to a string
    lol.push('l');
    println!("{}", lol);

    //define string and then append another to it
    //notice that push_str takes a literal (string slice)
    //that's on the stack. That prevents us from having to deal
    //with ownership issues.
    let mut s4 = String::from("foo");
    let bar = "bar";
    s4.push_str(bar);
    println!("S4 is {} and bar is {}", s4, bar);

    //concatenating with +
    let s5 = String::from("Hello");
    let s6 = String::from("world");

    //notice that we're passing ownership for s5 but
    //only borrowing for s6. s5 is no longer available
    //since s7 owns that string
    let s7 = s5 + &s6;

    //cannot pass s6 without a reference because rust
    //does + using fn add(self, s: &str) -> String {
    //which takes a reference
    //below would fail
    //let s7 = s5 + s6;

    //also cannot use the first string as a reference because the
    //add method requires self and not &self
    //let s7 = &s5 + &s6;

    println!("S7 is {}", s7);

    //bellow will not work as s5 was moved
    //s6 was using a reference so it was not moved and can be used
    println!("Can access s6 ({})", s6);
    //println!("Cannot access s5 ({}) any longer", s5);

    //adding multiple strings
    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");
    let tictactoe = s8 + "-" + &s9 + "-" + &s10;
    println!("{}", tictactoe);

    //adding using format
    let s11 = String::from("tic");
    let s12 = String::from("tac");
    let s13 = String::from("toe");

    //able to concatenate using format. instead of printing
    //to the screen, format returns a String
    //it's clearer to read
    let tictactoe1 = format!("{}-{}-{}", s11, s12, s13);
    println!("{}", tictactoe1);

    //but also does not take ownership of the first string
    println!("{},{},{}", s11, s12, s13);

    //cannot index strings
    let s14 = String::from("hello");
    //below fails because rust does not support indexing
    //let h = s14[0];

    //but can use slices on string literals
    let hello_english = "hello";
    let sub_string_english = &hello_english[0..4];
    println!("{}", hello_english);

    //but be careful. above is un UTF-8 and below in UNICODE
    //and they are different sizes
    let hello_russian = "Здравствуйте";
    let sub_string_russian = &hello_russian[0..4];
    println!("{}", sub_string_russian);

    //if you get this wrong, like slicing only part of the
    //character, it will panic
    //let sub_string_russian_1 = &hello_russian[0..3];

    //iterating over characters
    for c in "Зд".chars() {
        println!("{}", c);
    }

    //iterating over characters
    for cc in "ahoj".chars() {
        println!("{}", cc);
    }
}
