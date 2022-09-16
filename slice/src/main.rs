fn main() {
    let s = String::from("Hello World");

    //reference to a string slice where 0 is the
    //begining and 5 is the end
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    let s1 = String::from("hello");
    //start at 0 and to do 4
    let hell = &s1[..4];
    println!("{}", hell);

    let s2 = String::from("hello");
    //start at 1 and to to the end
    let ello = &s2[1..];
    println!("{}", ello);

    let word = String::from("hola world");
    let first_word = first_word(&word);
    println!("{}", first_word);
}

fn first_word(word: &String) -> &str {
    //convert to bytes
    let word_bytes = word.as_bytes();

    //iterate through the bytes
    for (i, &item) in word_bytes.iter().enumerate() {
        //if we sing a space return 0 - index
        if item == b' ' {
            return &word[0..i];
        }
    }

    //no space - return the whole thing
    &word[..]
}
