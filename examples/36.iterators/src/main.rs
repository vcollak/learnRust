#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 11,
            style: String::from("dress"),
        },
        Shoe {
            size: 10,
            style: String::from("dress"),
        },
    ];

    let my_shoes = shoe_by_size(shoes, 10);
    println!("{:?}", my_shoes);

    //shows is no longer available since shoe_by_size took ownership
    //println!("{:?}", shoes);
}

//takes the ownership of shoes
fn shoe_by_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    //into_iter() Creates an iterator from a value
    //The iterator returned by into_iter may yield any of T, &T or &mut T, depending on the context.
    //The iterator returned by iter will yield &T, by convention.
    //The iterator returned by iter_mut will yield &mut T, by convention.
    shoes.into_iter().filter(|s| s.size == size).collect()
}
