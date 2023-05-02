
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    //creating a immutable integer
    let immutable_int = 5;
    println!("the value of immutable_int is: {}", immutable_int);
    
    
    //creating a mutable integer
    let mut mutable_int = 5;
    print!("the value of mutable_int is: {}", mutable_int);
    mutable_int = 6;
    print!("the new value of mutable_int is: {}", mutable_int);

    //print the constant value
    println!("the value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    //shadowing
    let shadowed_int = 5;
    println!("the value of shadowed_int is: {}", shadowed_int);
    let shadowed_int = shadowed_int + 1;
    println!("the new value of shadowed_int is: {}", shadowed_int);

    //the shadowing work for change of type or new value

}
