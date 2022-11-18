use core::num;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    //list of rectangles
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    //closure that sorts by width
    //sort_by_key takes a function. We can use a closure
    //as a function. The closure has access to
    //the "r" value
    list.sort_by_key(|r| r.width);
    println!("{:?}", list);

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    //closure that first pushes some value to sort operations
    //vector for each item and also performs sort
    //the closure has access to vars above the closure
    list.sort_by_key(|r| {
        sort_operations.push(&value);
        r.width
    });

    println!("{:?}", sort_operations);
    println!("{:?}", list);

    //this both sorts by width, but also
    //counts the number of sorts
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("Number of sort operations: {}", num_sort_operations);
    println!("{:?}", list);

    //different ways to create a closure.
    //same behavior as the add_one function
    let add_one_v2 = |x: u32| x + 1;
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v4 = |x: u32| x + 1;

    println!("the value is: {}", add_one_v2(1));
    println!("the value is: {}", add_one_v3(2));
    println!("the value is: {}", add_one_v4(3));

    //this closure will take a String and return the same String
    let example_closure = |x| x;
    let s = example_closure(String::from("Hello"));
    println!("{s}");

    //cannot change the type of the the closure once the type
    //inference above completed
    //let n = example_closure(5);

    let list = vec![1, 2, 3];
    println!("Before defining closure {:?}", list);

    //can define the closure and execute at the same time
    // || just means that we're not passing parameters to a closure
    let only_borrows = || println!("From closure {:?}", list);

    println!("Before calling closure {:?}", list);
    //can also call previously defined closure as if it was a function
    only_borrows();
    println!("After calling closure {:?}", list);

    ///----
    let mut list1 = vec![1, 2, 3];
    println!("Before defining closure {:?}", list1);

    //can define the closure and execute at the same time
    let mut borrows_mutably = || list1.push(7);

    borrows_mutably();
    println!("After calling closure {:?}", list1);
}

fn add_one(x: u32) -> u32 {
    x + 1
}
