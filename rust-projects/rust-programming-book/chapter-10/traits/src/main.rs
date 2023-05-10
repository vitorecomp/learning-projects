// use aggregator::{Summary, Tweet};

use std::fmt::Display;

use traits::Tweet;
use traits::Summary;


//this functions defines that its parameter must implement the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}


//this is another way to define that its parameter must implement the Summary trait
//this is called trait bound, is better went is necessary to define that the call should
//implement multiples traits
pub fn notify_with_syntax_sugar<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn another_way(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}


//the last way allow for a clear way of annotating generics
//this will help to organize the code
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Summary,
{2}


//this syntax allow for the return of a object that implement a trait
//but its limited for a single object type being returned on any alternative
//meaning that is not possible to return a type on a if and another on the else
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.know_more());

    notify(&tweet);
}
