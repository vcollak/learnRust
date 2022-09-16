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

// front_of_house is private, but accessible
//frm the same level by eat_at_restaurant
mod back_of_house;
mod customer;
mod front_of_house;

fn deliver_order() {}
