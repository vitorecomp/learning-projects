use std::{thread, time::Duration};

pub enum MyType<T> {
    Some(T),
    None,
}


//the function that implement the FnOnce trait
//can be called at least one time
//the function could implement FnMut trait or Fn trait
//this traits will define the way to call the function
//and how many time they can be called
//on example below, the function can be called at least one time
impl<T> MyType<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            MyType::Some(t) => t,
            MyType::None => f(),
        }
    }
}

//another example is the sort from the standard library
//that will require the fnMut trait, because the function
// will use the closure multiple times to sort the vector

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    expensive_closure(5);


    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    add_one_v3(5);
    let add_one_v4 = |x| x + 1  ;
    add_one_v4(5);


    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);


    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();


    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    //this closure will not mutate or move anything meaning that
    //it will implement fill the requirements of FnMut trait
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
    

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    //this is a example of a function that will not satisfy the requirements
    //to implement the FnMut function, because it will move the "value" variable
    //to the out of its scope
    let noFnMut = |r : Rectangle| {
        sort_operations.push(value);
        r.width
    };



    //in this case the closure will satisfy the requirements to implement the FnMut
    //because it not move anything out of scope
    let mut sort_operations = vec![];
    list.sort_by_key(|r | {
        sort_operations.push(r.width);
        r.width
    });

    println!("-------");

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);

    
}


