//library that can generate random numbers
use rand::Rng;

//can compare things
use std::cmp::Ordering;

//io library to rake inputs
use std::io;

fn main() {
    //print "guess a number"
    println!("Guess the number!");

    //use random package to generate a random number in a range of 1 - 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //print the random (secret number)
    println!("The secret number is {secret_number}");

    //loop until we break  out of it
    loop {
        //let user input a guess
        println!("Please input your guess.");

        //define a new empty mutable string. mmut enables us to
        //change the string later
        let mut guess = String::new();

        //use io package to read a line from stdin. put the value
        //we get in the guess mutable string
        //notice that we need to use a mutable reference
        //if we didn't use a reference, read_line would own the guess
        //and guess would no longe be available here
        //read_line will return a Result enum so we need to
        //make sure we don't have error. If we do, return "failed to read line"
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //shadow the guess variable that's currently a string
        //to u32. guess.trim().parse() will first trim any
        //spaces and then parse the string to i32 (number)
        //if that works (determine match), return OK with a number
        //if it fails, continue the loop so user get to enter the value again
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //print the guess
        println!("You guessed: {guess}");

        //compare the guess to the secret number using match. match is like an if
        //again we pass secret number by reference so guess.cmp does not own it
        //print results.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
