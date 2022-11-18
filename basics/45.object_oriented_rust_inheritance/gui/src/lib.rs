pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    //holds a list of vectors that are
    //a type of box, which holds inside a dynamic trait
    //object called Draw
    //the cool thing about this is that we can add any struct that
    //implements draw. that means that we can share the library
    //and users can define their own components and still leverage the
    //Screen struct from the gui library
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        //call draw method for every component in the
        //components vector. We know the draw() is available
        //because component includes a trait object
        //trait objects allow for multiple concrete types to
        //fill in for the trait object at runtime
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

//implements the Draw trait for button struct
impl Draw for Button {
    fn draw(&self) {
        //implements draw for button
        println!("draw code in button");
    }
}
