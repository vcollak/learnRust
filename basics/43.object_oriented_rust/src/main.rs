//struct is like an object in that it holds
//properties
struct Person {
    name: String,
}

//struct can also have implementations so
//it can also have methods that act on those properties
impl Person {
    //this is basically a constructor. by convention
    //it's called new
    fn new(person: &str) -> Person {
        Person {
            name: String::from(person),
        }
    }

    //method to return a copy of the name
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    //instantiate a new object
    let person = Person::new("Dude");

    //call methods and access properties
    println!("Name by method: {}", person.get_name());
    println!("Name by property: {}", person.name);
}
