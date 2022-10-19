//enum that holds various types
enum Message {
    Quit,                       //no type
    Move { x: i32, y: i32 },    //2 named fields
    Write(String),              //string
    ChangeColor(i32, i32, i32), //three values (tuple)
}

//implementation of Message enum with methods
impl Message {
    //call method attached to the enum
    fn call(&self) {
        //something happens here
    }
}

fn main() {
    //create a message with type write and string
    let m = Message::Write(String::from("Hello"));

    //call the "call" method of the Message enum
    m.call();
}
