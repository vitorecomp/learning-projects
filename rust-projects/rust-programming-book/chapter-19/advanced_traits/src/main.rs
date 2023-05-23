struct Counter {
    count: u32,
}

//different from a generic type parameter
//the associated type allow for the usage of the method
//next with having to annotate the return type
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        return Some(32);
    }
}

//is impossible to implment the same trait for different types
//when using associated types
// impl Iterator for Counter {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {
//         // --snip--
//         return Some(32);
//     }
// }

pub trait IteratorHybrid<T> {
    fn nextNew(&mut self) -> Option<T>;
}

impl IteratorHybrid<String> for Counter {
    fn nextNew(&mut self) -> Option<String> {
        return Some("Teste".to_string());
    }
}

impl IteratorHybrid<u32> for Counter {
    fn nextNew(&mut self) -> Option<u32> {
        return Some(32);
    }
}

///////
// The usage of the default generic type
//allow for the usage associated types with generic types
//that implement the trait
use std::{ops::Add, fmt};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}


//chossing what implmentation to call

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

//for static methods
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// this will determe that to use the OutlinePrint
//the class should implement first the fmt::Display
//trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct SimplePoint {
    x: i32,
    y: i32,
}


impl fmt::Display for SimplePoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//this will just be possible if the class fist implement the fmt::Display
impl OutlinePrint for SimplePoint {}


//to overcome the orphan rule in some situations is possible to make use
//of the newtype pattern this is done by encapsulation the type in a new
//wrapper type

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    println!("Hello, world!");

    let mut counter = Counter { count: 1 };
    //in case of associated types is not necessary to define which type the return will be
    let number = counter.next();

    //in case of a generic is necessary to anotate the type of the return
    let number2: Option<u32> = counter.nextNew();

    //testing the sum
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );


    //in case of multiples traits having the same method
    //is possible to use the follow syntax to chose which on to call


    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);

    //it will default to the class implementation
    //in case it not exists the compiler will throw an error
    person.fly();


    println!("A baby dog is called a {}", Dog::baby_name());
    //the call below is impossible because animal is a abstract
    //class
    // println!("A baby dog is called a {}", Animal::baby_name());

    //to call the default implementation of a static method
    //is necessary the usage of the follow syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());


    //using a new display type on vec
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    //this pattern have its limitation
    //it will not make avaible all the methods from the vec
    //to do it is necessary to implement the deref trait



}
