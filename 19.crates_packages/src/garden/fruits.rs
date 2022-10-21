#[derive(Debug)]

//needs to be pub to import be visible from
//outside of this crate
pub struct Strawberry {
    pub name: String,
}

//implements the Strawberry
impl Strawberry {
    //create a new instance of Strawberry and return it
    pub fn new(name: String) -> Self {
        Strawberry { name: name }
    }

    //return the name
    pub fn get_name(&self) -> String {
        //TODO fix clone
        //not all that efficient to clone
        self.name.clone()
    }
}
