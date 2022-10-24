/*

Library that is structured to use several modules. Some modules have their
own sub-modules as follows:

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
    -back_of_house
        |__ fix_incorrect_order
        |__ cook_order

Items in a parent module can’t use the private items
inside child modules (must be pub), but items in child modules can use the items
in their ancestor modules.


Bring in the back_of_house, customer, front_of_house modules
These modules are private, but accessible because they are at the
same level as the library (lib.rs) and not a subfolder
*/
mod back_of_house;
mod customer;
mod front_of_house;

//the root library has the deliver_order function
fn deliver_order() {}
