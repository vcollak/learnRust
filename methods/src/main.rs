#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//implementation of rectangle that
//includes methods
impl Rectangle {
    //area method
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
    //the above can_hold(&self...) is just a shorthand for this
    fn can_hold_self(self: &Self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //returns the rectangle type that a square
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

    if rect.width() {
        println!("The rectangle has a width of: {}", rect.width);
    }

    if rect.can_hold(&rect_smaller) {
        println!("Rect can hold rect_smaller");
    }

    if !rect.can_hold(&rect_larger) {
        println!("Rect cannot hold rect_larger");
    }

    let square = Rectangle::square(10);
    println!("Area of the square is {}", square.area());
}
