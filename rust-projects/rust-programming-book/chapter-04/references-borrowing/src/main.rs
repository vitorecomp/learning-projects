fn main() {
    let mut s1 = String::from("hello");

    //by that way the calculate_length function not take the ownership of the variable s1
    //the variable s1 is borrowed by the function calculate_length
    //something borrowed is not owned, and by default immutable
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    //to allow change is necessary to explicit declare the variable as mutable
    change(&mut s1);

    //one of the caviats of the borrowing is that only one mutable reference is allowed
    //so the code below will give a error not allowing for the second mutable reference
    //let r1 = &mut s;
    //let r2 = &mut s;
    //this is done to avoid race conditions and that the variable is not changed by other
    // while being used by another

    //the code above will work if each variable is inside its own scope
    {
        {
            let r1 = &mut s1;
            println!("{}", r1);
        }
        let r2 = &mut s1;
        println!("{}", r2);
    }
    
    //this is possible because the borrow of r1 will end with the end of the scope
    
    //the exception to the rule of only immutable reference is the case of immutable
    let r1 = &s1;
    let r2 = &s1;
    // let r2 = &mut s1; this code will be impossible to run, because the variable r1 is borrowed as immutable


    //now the code below will not work because the variable r1 is borrowed as immutable
    println!("{}", r1);
    println!("{}", r2);
    //but now after use is possible to reborrow the variable as mutable
    let r3 = &mut s1;
    println!("{}", r3);

    


}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
