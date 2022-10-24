/*
Bring the hosting from the front of the house into the scope of the customer.
We can do that becuase hosting is defined as pub using "pub mod hosting;" in front_of_house

We could bring use crate::front_of_house::hosting::add_to_waitlist
to scope so we can just call add_to_waitlist() below,
but that would not be idiomatic. We want to bring in the module.
The pub keyword allows us to re-export the module to those who use
the customer module. Now external code can also use hosting::add_to_waitlist()


*/

//re-export to those who use the customer module
pub use super::front_of_house::hosting;

//pub makes it accessible from root lib
pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    super::front_of_house::hosting::add_to_waitlist();

    //because we have done use crate::front_of_house::hosting;
    //we can do this
    hosting::add_to_waitlist();

    let mut meal = super::back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    //meal.seasonal_fruit = String::from("blueberries");
}
