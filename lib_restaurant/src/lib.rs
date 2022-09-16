/*
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

Items in a parent module can’t use the private items
inside child modules, but items in child modules can use the items
in their ancestor modules.

*/

fn deliver_order() {}

// front_of_house is private, but accessible
//frm the same level by eat_at_restaurant
mod front_of_house {

    //hosting must be made public in order the ancestors like eat_at_restaurant
    //to access it
    pub mod hosting {

        //this also has to be made public so eat_at_restaurant can access it
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {

    //all values in the enum are public once enum is public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    //struct can be public, but only fields that are
    //pub will be visible
    pub struct Breakfast {
        //toast is public and can be called from parent
        pub toast: String,

        //seasonal fruit is private by default
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();

        //since the deliver order is in the parent
        //we add super to the path
        super::deliver_order();
    }

    fn cook_order() {}
}

mod customer {

    //bring the hosting into the scope of the eat_at_restaurant
    //we could bring use crate::front_of_house::hosting::add_to_waitlist
    //to scope so we can just call add_to_waitlist() below,
    //but that would not be idiomatic. we want to bring in the module
    //the pub keyword allows us to re-export the module to those who use
    //the customer module. Now external code can also use hosting::add_to_waitlist()
    pub use crate::front_of_house::hosting;

    //However for enums or structs, we want to bring in those into
    //the scope and not just the parent module use std::collections
    use std::collections::HashMap;

    //brinding in the result type into scope
    use std::fmt::Result;

    //since we cannot have the same result from different modules
    //in the scope, use "as" keyword to rename it to IoResult
    use std::io::Result as IoResult;

    //can bring multiple modules from the same path (std). It's the same as
    //doing these two
    //  use std::cmp::Ordering;
    //  use std::io;
    use std::{cmp::Ordering, io};

    //to bring in these two we can do below
    //  use std::vec;
    //  use std::vec::Drain;
    use std::vec::{self, Drain};

    //bring in all public items from the collections scope
    //Be careful when using the glob operator! Glob can make it harder to tell
    //what names are in scope and where a name used in your program was defined.
    use std::collections::*;

    pub fn eat_at_restaurant() {
        let mut map = HashMap::new();
        map.insert(1, 2);

        //absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        //relative path
        super::front_of_house::hosting::add_to_waitlist();

        //becuase we have done use crate::front_of_house::hosting;
        //we can do this
        hosting::add_to_waitlist();

        let mut meal = super::back_of_house::Breakfast::summer("Rye");

        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        //meal.seasonal_fruit = String::from("blueberries");
    }
}
