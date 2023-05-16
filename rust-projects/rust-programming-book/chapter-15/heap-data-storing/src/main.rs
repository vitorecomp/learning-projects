
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);

    //this is show 5 integers stored on the heap
    println!("b = {}", b);

    //enabling cons list ()
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
