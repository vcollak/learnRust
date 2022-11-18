//we use the Draw trait form the gui library
use gui::Draw;

//we import the screen and button from the gui library
use gui::{Button, Screen};

//we're bringing in our own component that we can add to other components
//from the gui library like Screen
//we're not using the properties if SelectBox anywhere so we added
//#[allow(dead_code)]to prevent the warning
#[allow(dead_code)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

//we can implement the trait for SelectBox
impl Draw for SelectBox {
    fn draw(&self) {
        //code to draw select box
        println!("draw code in select box");
    }
}

fn main() {
    //define a screen (from the gui library) that includes
    //a vector of components
    //we have 2 components: SelectBox and Button
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            //below will not work. We can only add types that
            //implement the Draw trait
            //Box::new(String::from("Hello")),
        ],
    };

    //run method on the screen iterates through the components
    //list and calls the draw method. each component must have the
    //draw method because it implements the Draw trait
    screen.run();
}
