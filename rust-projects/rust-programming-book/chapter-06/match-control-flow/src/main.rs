enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

fn main() {
    println!("Hello, world!");

    //this asslo for the use f the function, and there the none will be treated
    // by a match, the match should treat all the possible cases
    //if not the compiler will throw an error
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    //another example of match is the usage of other
    //this will be used when the match is not exhaustive
    //allow for a default case to be used
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    //another example in  match is the usage of _
    //this will suppress any warning of non used variables
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    
}
