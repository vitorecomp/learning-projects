
fn main() {
    println!("Hello, world!");
    let mut number = 3;

    if number < 5{
        println!("condition was true");
    }else{
        println!("condition was false");
    }

    //to use it the booth returns should be the same type
    let another_number = if number <= 5 { 5 } else { 6 };
    println!("another_number is {}", another_number);

    loop {
        println!("again my first loop!");
        number-=1;
        if number == 0 {
            break;
        }
    }

    number = 3;

    //is possible to return a value on the loop
    let mut run_time = 0;
    let mut even_runs = loop {
        run_time+=1;
        println!("again with even verification!");
        number-=1;
        if number == 0 {
            break run_time%2 == 0;
        }
    };
    
    println!("even_runs is {}", even_runs);

    number = 3;

    'main_loop: loop {
        loop {
            println!("again with condition break!");
            number-=1;
            if number == 0 {
                //is possible to the the main loop from here
                break 'main_loop;
            }
        }
    }

    //loop with conditions
    number = 3;
    while number > 0 {
        number-=1;
        print!("{} ", number);
    }

    //loop though a collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //a better way to loop on a array
    for element in a {
        println!("the value is: {}", element);
    }

    //even that whiles are good for loop though a number of times
    //most rustians use for loops with range

    for number in (1..4).rev() {
        println!("{number}!");
    }

}
