//define a trait. It's like an interface
//in other languages
pub trait Summary {
    //list methods the trait must implement
    fn summarize(&self) -> String;
}

//type NewsArticle
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//implements the trait Summary for the NewsArticle struct
impl Summary for NewsArticle {
    //implements summarize as required by the trait
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    /* cannot implement this method since it's not defined by the trait
    fn authorize(&self) -> String {
        format!("Authorize thyself")
    }
    */
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//implements Summary trait for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

fn main() {
    //define a tweet
    let tweet = Tweet {
        username: String::from("daDude"),
        content: String::from("Some content"),
        reply: false,
        retweet: false,
    };

    //call tweet's summarize impl
    println!("1 new tweet {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Top News"),
        location: String::from("Houston"),
        author: String::from("daDude"),
        content: String::from("New content"),
    };

    //call news article impl
    println!("1 new article {}", article.summarize());
}
