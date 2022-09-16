//bring the hosting into the scope of the eat_at_restaurant
//we could bring use crate::front_of_house::hosting::add_to_waitlist
//to scope so we can just call add_to_waitlist() below,
//but that would not be idiomatic. we want to bring in the module
//the pub keyword allows us to re-export the module to those who use
//the customer module. Now external code can also use hosting::add_to_waitlist()
pub use super::front_of_house::hosting;

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
