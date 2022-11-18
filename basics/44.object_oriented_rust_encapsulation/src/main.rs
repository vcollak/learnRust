//the struct is public so it's accessible outside of this module
//but the fields are private and not accessible.
//we can only update them via methods
//this way AverageCOnnection can only
//be manipulated by the methods and average does not get out of sync
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    //creates a new average collection
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        }
    }
    //add a value to the list and update average
    //we're changing the value so the self reference has to be
    //mutable
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    //removes the last entry from the list (pop)
    //also need to pass mutable reference since we're
    //changing the list
    //return the option so we can determine is it succeeded or not
    //the self.list.pop() also returns an option
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            //we were able to remove. update average
            Some(value) => {
                self.update_average();
                Some(value)
            }
            //unable to remove. return none
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }
}

fn main() {
    //have to define as mutable since the methods of the
    //list take a mutable reference
    let mut my_list: AveragedCollection = AveragedCollection::new();
    my_list.add(10);
    my_list.add(20);
    my_list.add(30);
    my_list.remove();
    println!("Average is {}", my_list.average);

    //this is possible only because we're in the same module
    //externally the average property would not be public
    //my_list.average = 2.0;
}
