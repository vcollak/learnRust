#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

//module that implements tests. We can call cargo run to execute
// the tests automatically
#[cfg(test)]
mod tests {

    //we need to be able to panic to exit a test when it fails
    use core::panic;

    //need to make sure we have access to Rectangle and various methods
    use super::*;

    //testing the add function
    #[test]
    fn add_test() {
        //call the add method
        let result = add(2, 2);

        //assert that result from the method is 4
        //if not, it fails the test
        assert_eq!(result, 4);
    }

    //flat out forcing a fail of a test
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    //define 2 rectangle and testing if larger_can_hold_smaller
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        //assert that the "larger" Rectangle can hold the smaller one
        assert!(larger.can_hold(&smaller));
    }

    //testing the opposite that smaller cannot hold larger one
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // "!" means Not. smaller Rectangle can_hold NOT the larger
        assert!(!smaller.can_hold(&larger));
    }
}
