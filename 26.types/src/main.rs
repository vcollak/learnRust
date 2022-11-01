use core::panic;

//a type called Guess with a value of i32
struct Guess {
    value: i32,
}

//implements Guess
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
    //thanks to &self(which is shorthand for (self: &Self)
    //this is a method attached to the instance of the Guess
    fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    //instantiate new type

    //using :: because it's a static function on the struct
    //and not a method on the object
    let guess = Guess::new(10);

    //call the method
    println!("{}", guess.value());
}
