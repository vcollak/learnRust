fn main() {
    let v1 = vec![1, 2, 3];

    //created an iterator
    let v1_iter = v1.iter();

    //looped over the iterator
    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1_iter_new = v1.iter();
    let total: i32 = v1_iter_new.sum();
    println!("{}", total);

    //define a new vector. assign the v1 iterator to it
    //using a closure that adds 1 to each element
    //prints [2,3,4]
    //collect takes the results and put the in the collection
    //without that nothing happens because iterators are lazy
    //(nothing happens until they are called
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);
}
