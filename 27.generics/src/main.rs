//function takes a generic value &[T] as a parameter
fn largest<T>(number_list: &[T]) {
    //sone functionality that handles the largest
}

//generic struct with one parameter type
struct Point<T> {
    x: T,
    y: T,
}

//methods can leverage generics too
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

struct PointA<T, U> {
    x: T,
    y: U,
}

//enum with generics
enum Option<T> {
    Some(T),
    None,
}

//multiple types
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    //point where all params are the same type
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 10.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());

    //mixing integers and floats
    let mix = PointA { x: 1, y: 10.0 };
}
