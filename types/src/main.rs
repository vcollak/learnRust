use core::panic;

//a type called Guess with a value
struct Guess {
    value: i32,
}

impl Guess {
    //return a new guess. basically a constructor
    fn new(value: i32) -> Guess {
        //validate first
        if value < 1 || value > 100 {
            panic!("Value must be between 1 - 100");
        }

        //return
        Guess { value }
    }

    //method to return the actual value
    fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    //instantiate new type
    let guess = Guess::new(10);
    println!("{}", guess.value());
}
