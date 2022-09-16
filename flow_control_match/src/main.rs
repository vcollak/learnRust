//enum that lists states and can be printed
#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    Texas,
}

//types os coins
//the quater can specify it's from a UsState
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    //This seems very similar to an expression used with if,
    //but there’s a big difference: with if, the expression needs to
    //return a Boolean value, but here, it can return any type
    match coin {
        //can have any kind of expression
        Coin::Penny => {
            println!("Pretty Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,

        //can also assign the enun value after the match
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    //call value_in_cents and pass the coin type with state
    let value = value_in_cents(Coin::Quarter(UsState::Texas));
    println!("The value of the quarter is: {}", value);
}
