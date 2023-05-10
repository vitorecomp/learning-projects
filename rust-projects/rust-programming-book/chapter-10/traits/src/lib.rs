pub trait Summary {

    //the semicolon after the method signature indicates that we’re
    //implementing a trait rather than defining a method body
    //this will oblige the implementor to define the body of the method
    fn summarize(&self) -> String;


    //this will provide a default implementation for the method
    fn know_more(&self) -> String {
            format!("(Read more from {}...)", self.summarize())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    //its possible to override the default implementation
    fn know_more(&self) -> String {
        format!("(Read more from the new article {}...)", self.summarize())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

