//rectangle struct
#[derive(Debug)] //enables us to use the println! macro for rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    //define the rectangle
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    //print rectangle calling the area function
    //we are passing by referneces to we don't move the
    //rect1 to area since we want to use it later
    println!("The area of the rectangle is {} square pixels", area(&rect));

    //can still use the rect1 because we just borrowed it in area
    println!("{}", rect.height);

    //cannot print the rectangle without it implementing a Display trait
    //println!("rest is {}", rect);

    //:? users a DEbug trait to print the struct
    println!("rest is {:?}", rect);

    //:#? is another way to do the same
    println!("rest is {:#?}", rect);

    //another way to print. thi sone goes to stderr instead of
    //stdout (like println! macro)
    //this one also prints the line number - neat!
    dbg!(&rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
