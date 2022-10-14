//User struct that defines several user fields
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//function that creates a new User struct and returns it
fn new_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

//using shorthand since the field name and parameter are the same
fn new_user_short(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    //Note that the entire instance must be mutable;
    //Rust doesn’t allow us to mark only certain fields as mutable.
    let mut user = User {
        email: String::from("user@test.com"),
        username: String::from("user"),
        active: true,
        sign_in_count: 1,
    };

    //mutate user
    user.email = String::from("daUser@test.com");

    //build user using the new_user function
    let email = String::from("john@test.com");
    let username = String::from("john");
    let user1 = new_user(email, username);
    println!("{}", user1.email);

    //create a new user, but use some fields from another user
    let user2 = User {
        email: user.email,
        username: user.username,
        active: false,
        sign_in_count: 0,
    };

    //this will define email and
    //use the rest of the fields from user2
    let user3 = User {
        email: String::from("jane@test.com"),
        ..user2 //this moves the ownership from user2 to user3
    };

    //this works because active is on the stack and was copied
    println!("{}", user2.active);

    //this will fail because the reference from user2 was moved to user3
    //and username is on the heap
    //println!("{}", user2.username);

    //define a tuple struct
    //Note that the black and origin values are different types,
    //because they’re instances of different tuple structs.
    //Each struct you define is its own type, even though the fields
    //within the struct might have the same types.
    //For example, a function that takes a parameter of type Color cannot
    //take a Point as an argument, even though both types are made up of three i32
    //values.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
