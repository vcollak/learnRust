use std::sync::Arc;

//default trait where we implement directly in the trait
pub trait Summary {
    //must be implemented by all that implement the trait
    fn summarize_author(&self) -> String;

    //definition and default implementation for those that
    //have not implemented separately
    //this can also call the function of the implementation
    //but implementation cannot call default
    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

//still need an empty implementation
impl Summary for NewsArticle {
    //NewsArticle implementation
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//implements Summary trait for Tweet
//this overrides the default trait implementation
impl Summary for Tweet {
    //tweet implementation
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

//item can be any type that implements the trait
//an alternative to this implementation would be:
//  pub fn notify<T: Summary>(item: &T) {}
//can also force that the type must implement both Summary and Display traits
//  pub fn notify(item: &(impl Summary + Display)) {}
//one more alternative is to use where. This is useful to make the
//method signature a bit more readable since we have multiple params
//and each has to implement different traits:
// fn some_function<T, U>(t: &T, u: &U) -> i32
//    where T: Display + Clone,
//          U: Clone + Debug{}
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//instead of concrete type like Tweet or NewsArticle, this returns the
//trait. As a result, we can return any type that implements a trait
//but have to always a single type. Cannot return Tweet or NewsArticle
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    //define news article and call summarize
    //since the NewsArticle does not implement Summarize, we're
    //calling the default function implemented in the trait
    let news = NewsArticle {
        headline: String::from("Hola"),
        location: String::from("Houston"),
        author: String::from("DaDude"),
        content: String::from("Some news"),
    };

    println!("{}", news.summarize());

    //define a tweet and call summarize
    let tweet = Tweet {
        username: String::from("daDude"),
        content: String::from("Some content"),
        reply: false,
        retweet: false,
    };

    //call tweet's summarize impl
    println!("1 new tweet {}", tweet.summarize());

    //call the function that implements
    //a parameter that's any type that implements the
    //trait
    notify(&tweet);
    notify(&news);
}
