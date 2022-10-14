use crate::List::{Cons, Nil};

enum List {
    //instead of i32 and List (recursive), wrap it around a box
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    //box is a pointer to data that's stored
    //in the heap
    let b = Box::new(5);
    println!("b = {}", b);

    //below implements a list that hosts other lists
    //each list holds a i32 and another list
    //if we just did this, the compiler will not know
    //how much space to reserve in the stack
    //the box forces that to be done in a heap
    //    let list = Cons(1, (Cons(2, (Cons(3, Nil)))));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
