fn another_function() {
    println!("this is another function");
}

fn label(value : i32, label : char) {
    println!("the value is {} and the label is {}", value, label);
}

fn sum(a : i32, b : i32) -> i32 {
    //this will be the return value
    a + b

    //its also possible to explicit define the return value
    // return a+b;
}

fn main() {
    println!("this is the main function");
    another_function();
    label(5, 'a');
    println!("the sum of 5 and 6 is {}", sum(5, 6));
}
