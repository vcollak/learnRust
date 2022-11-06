fn main() {
    let v1 = vec![1, 2, 3];

    //Returns an iterator over the slice.
    //The iterator yields all items from start to end.
    let v1_iter = v1.iter();

    //looped over the iterator
    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1_iter_new = v1.iter();

    //Sums the elements of an iterator.
    //Takes each element, adds them together, and returns the result.
    //An empty iterator returns the zero value of the type.
    let total: i32 = v1_iter_new.sum();
    println!("{}", total);

    //Define a new vector. Assign the v1 iterator to it with iter()
    //map() takes a closure and creates an iterator which calls that closure on each element
    //using a closure that adds 1 to each element
    //collect takes the results and put the in the collection
    //without calling collect nothing happens because iterators are lazy
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    //prints [2,3,4]
    println!("{:?}", v2);
}
