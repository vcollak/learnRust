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
