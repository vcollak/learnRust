#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//implementation of rectangle that
//includes methods
impl Rectangle {
    //area method. notice that it takes a &self
    //reference as a parameter so it has access to struct fields
    //also notice this can infer the type of &Self (see can_hold_self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //return true if the width is more than 0
    fn width(&self) -> bool {
        self.width > 0
    }

    //sends another rectangle and returns true if
    //self can hold it inside of it (is larger)
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //another way to express the method signature. in fact,
    //the above can_hold(&self) is just a shorthand for this
    fn can_hold_self(self: &Self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //returns the rectangle type that a square
    //does not have self as a parameter since it does not
    //need to access to the instance and its fields.
    //returns Self type
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect_smaller = Rectangle {
        width: 15,
        height: 5,
    };

    let rect_larger = Rectangle {
        width: 300,
        height: 500,
    };

    println!("The area of the rectangle is {} square pixels", rect.area());

    //call the method of the implementation of Rectangle struct (aka object)
    if rect.width() {
        println!("The rectangle has a width of: {}", rect.width);
    }

    if rect.width() {
        println!("The rectangle has a width of: {}", rect.width);
    }

    if rect.can_hold(&rect_smaller) {
        println!("Rect can hold rect_smaller");
    }

    if !rect.can_hold(&rect_larger) {
        println!("Rect cannot hold rect_larger");
    }

    //call the method of the struct (not the implementation / object)
    //notice we use :: instead of . since this is an associated function
    //and not a method on the object
    let square = Rectangle::square(10);
    println!("Area of the square is {}", square.area());
}
