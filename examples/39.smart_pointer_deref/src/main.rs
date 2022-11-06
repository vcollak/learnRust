/*
Used for immutable dereferencing operations, like *v.
In addition to being used for explicit dereferencing operations with the
(unary) * operator in immutable contexts, Deref is also used implicitly
by the compiler in many circumstances. This mechanism is called 'Deref coercion'.
In mutable contexts, [DerefMut] is used.
Implementing Deref for smart pointers makes accessing the data behind them convenient,
which is why they implement Deref. On the other hand, the rules regarding Deref and [DerefMut]
 were designed specifically to accommodate smart pointers.
 Because of this, Deref should only be implemented for smart pointers to avoid confusion.

For similar reasons, this trait should never fail.
Failure during dereferencing can be extremely confusing when Deref is invoked implicitly.
 */
use std::ops::Deref;

//struct with generic parameter
struct MyBox<T>(T);

//implements Deref trait for MyBox so we can de-reference *y in main
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//implements box with new method returning the new MyBox
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    println!("X is {}", x);

    //to access the value need to de-reference MyBox
    println!("Y is {}", *y);

    let z = MyBox::new(String::from("Hello"));
    println!("z is {}", *z);
}
