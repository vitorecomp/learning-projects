fn main() {

    //the fist match case is the simpler
    //the usage is to decide between the enum
    //option
    let x : Option<i32> = Some(10);
    match x {
        Some(i) => println!("Got {}", i),
        None => println!("Got nothing"),
    }

    //the second possible usage are in the if statement
    //this is useful, when implementing a function
    //that will f=have a default behaviour for other
    //values that are the if value
    if let Some(i) = x {
        println!("Got {}", i);
    } else {
        println!("Got nothing");
    }


    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    //is also possible use it in the same way on a loop
    //where it will enter in the loop if the type
    //is matched
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];


    //the last usages match is possible to make usage of it
    // as a destruct pattern allowing the opening of tuples
    //in variables, really useful when working with tuples
    //the first example of it is in the for loop where we open 
    //the enumerate tuple
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    //using to deconstruct tuples in variables
    let (x, y, z) = (1, 2, 3);

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    //extremely useful when working with functions
    let point = (3, 5);
    print_coordinates(&point);

    
}