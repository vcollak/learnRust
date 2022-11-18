fn main() {
    let config_max = Some(3u8);

    //can match using the match keyword
    //but will have to add _ => () to exhaust all possibilities
    match config_max {
        Some(max) => println!("Max is configured to be {}", max),
        _ => (),
    }

    //alternatively, we can use if let
    //you can think of if let as syntax sugar for a match that
    //runs code when the value matches one pattern and then
    //ignores all other values
    if let Some(max) = config_max {
        //this only runs if config_max is of type Some(u8)
        //if so, it will assign config max value to max
        println!("Max is configured to be {}", max);
    }
}
