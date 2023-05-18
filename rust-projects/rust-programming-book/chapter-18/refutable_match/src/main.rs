fn main() {
    let x : Option<u32> = None;

    //this is a problem because we try to use a refutable
    //match option on a irrefutable pattern
    // let Some(y) = x;

    match x {
        Some(y) => println!("y = {}", y),
        None => println!("None"),
    }

    //another way is the usage of if let
    if let Some(x) = x {
        println!("{}", x);
    }

    //the inverse is possible, meaning that is
    //the compiler will accept irrefutable code
    //in a refutable if, the only point is that it will appoint it
    //as a warning
    if let x = 5 {
        println!("{}", x);
    };
}
