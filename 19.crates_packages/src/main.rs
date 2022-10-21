//bring asparagus struct into scope
//from the garden::vegetables
//the garden crate includes vegetables sub-module
use crate::garden::vegetables::Asparagus;

//get Strawberry from fruits
use crate::garden::fruits::Strawberry;

//include the garden module in the main binary crate
//garden includes both Fruits and Vegetables
pub mod garden;

fn main() {
    //define plant as asparagus
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);

    //create a new strawberry
    let fruit: Strawberry = Strawberry::new(String::from("Very Berry"));

    //print results of the get_name method
    println!("{}", fruit.get_name());
}
