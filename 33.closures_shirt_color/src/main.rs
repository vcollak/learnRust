#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

//inventory of shirts
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    //the user can either have a preference or not. As a result
    //we pass an Option<ShirtColor>, which an be ether ShortColor or None
    //regardless, we return what ShirtColor we give away
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        //unwrap_or_else either returns the value of the Option (if it's set)
        //or calls the closure. In this case self.most_stocked()
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    //finds what shirts we have the most of
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        //iterate through the inventory
        //use a reference so we don't move ownership
        //to this method
        for color in &self.shirts {
            //match red or blue and add the count
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        //return the one that we have the most of
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    //set the inventory
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    //user prefers Red. Notie that we need to wrap ShirtColor::Red an Option
    let user_pref1 = Some(ShirtColor::Red);
    //run the giveaway by passing the preference
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    //no preference
    let user_pref2 = None;

    //run the giveaway by passing the preference~~
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
