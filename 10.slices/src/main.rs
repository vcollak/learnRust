fn main() {
    let s = String::from("Hello World");

    //reference to a string slice where 0 is the
    //beginning and 5 is the end
    let hello = &s[0..5];

    //6 - 11
    let world = &s[6..11];

    //as always, be careful with indexes. below will panic,
    //but only in runtime
    //let fail = &s[20..30];

    println!("{} {}", hello, world);

    let s1 = String::from("hello");
    //start at 0 and go do 4
    let hell = &s1[..4]; //"hell"
    println!("{}", hell);

    let s2 = String::from("hello");
    //start at 1 and to to the end
    let ello = &s2[1..]; //"ello"
    println!("{}", ello);

    let word = String::from("hola world");
    let first_word = first_word(&word);
    println!("{}", first_word);
}

//converts a string to bytes and then
//iterates over the bytes until it sees the space
//it then returns the string till the space
//in reality we could have used split_whitespace function on the string
fn first_word(word: &String) -> &str {
    //convert to bytes
    let word_bytes = word.as_bytes();

    //iterate through the bytes
    for (i, &item) in word_bytes.iter().enumerate() {
        //if we see a space, return 0 - index
        if item == b' ' {
            return &word[0..i];
        }
    }

    //no space - return the whole thing
    &word[..]
}
