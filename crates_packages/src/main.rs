//bring asparagus struct into scope
use crate::garden::vegetables::Asparagus;

//include the garden module in the main binary crate
pub mod garden;

fn main() {
    //define plant as asparagus
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
}
