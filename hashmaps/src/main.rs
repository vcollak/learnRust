use std::collections::HashMap;

fn main() {
    //Like vectors, hash maps are homogeneous: all of the keys must
    // have the same type as each other, and all of the values must
    // have the same type.
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let team_name = String::from("blue");

    //this returns an option so need to match before we can se it
    let score = scores.get(&team_name);
    match score {
        Some(score) => println!("Team {} score is {}", team_name, score),
        None => println!("No score for team {}", team_name),
    }

    //another way to do this is unwrap_or. In this case
    //if the value is Some() we get the score. Otherwise,
    //we get a default of &0.
    let score_blue = scores.get(&team_name).unwrap_or(&0);
    println!("Team {} score is {}", team_name, score_blue);

    //below will panic. we're getting a value that does not exist
    //and instead of checking for Some or None in match
    //or using unwrap_or, unwrap will panic is the key "red" does not exist
    //let score_blue1 = scores.get("red").unwrap();

    //iterating over hashmap
    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    ////////////////////////////
    // Hash Maps and Ownership
    ////////////////////////////
    //For types that implement the Copy trait, like i32, the values are copied
    //into the hash map. For owned values like String, the values will be moved
    //and the hash map will be the owner of those values

    let field_name = String::from("favorite_color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    //the ownership for field_name, field_value were moved into
    //the hashmap so can no longer do this
    //println!("{}:{}", field_name, field_value);

    //but we can always use references to make sure ownership
    //does not change
    let field_name_r = String::from("favorite_color");
    let field_value_r = String::from("blue");

    let mut map_r = HashMap::new();
    map_r.insert(&field_name_r, &field_value_r);
    println!("{}:{}", field_name_r, field_value_r);

    //If we insert a key and a value into a hash map and then insert that same key
    //with a different value, the value associated with that key will be replaced.
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("blue"), 10);
    scores1.insert(String::from("blue"), 25);
    println!("{:?}", scores1);

    //inserting or changing the value only if the key exists
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("blue"), 10);

    //this will not change the blue team score, because the key already exists
    scores2.entry(String::from("blue")).or_insert(50);

    //the yellow key does not exist so it will insert a new record / entry
    scores2.entry(String::from("yellow")).or_insert(50);

    println!("{:?}", scores);

    ///////////////////////////
    // updating HashMap values
    ///////////////////////////

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    //iterate over the slice of the text
    for word in text.split_whitespace() {
        //or_insert will get a reference to an entry by key. if does not exist
        //just insert 0
        let count = map.entry(word).or_insert(0);

        //dereference and update the count
        *count += 1;
    }

    println!("{:?}", map);
}
