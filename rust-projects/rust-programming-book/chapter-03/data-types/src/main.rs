fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);
    // same functions will make necessary to use type annotation
    // the, this i necessary because the function will make necessary
    // the definition of this type for the own implementation 
    // let guess: (u32) -> F = "42".parse().expect("Not a number!");


    // Litetals
    let x = 9_000_000; // a better way to write 9 millions
    println!("The value of x is: {}", x);
    let x: i32 = 0xff; // the value can also be written in hex
    println!("The value of x is: {}", x);
    let x: i32 = 0o77; // the value can also be written in octal
    println!("The value of x is: {}", x);
    let x: i32 = 0b1111_0000; // the value can also be written in binary
    println!("The value of x is: {}", x);
    let x: u8 = b'A'; // the value can also be written in byte
    println!("The value of x is: {}", x);

    // the types bool, char, and all the numeric types are scalar types

    //some new type of variables that is important to know
    // is the tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {}", tup.0);
    println!("The value of tup is: {}", tup.1);
    println!("The value of tup is: {}", tup.2);

    //this will allow for the storage of multiple values in the same var

    //array
    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {}", a[0]);
    //this will understand that the array have at least 5 elements
    // that can also be declarated like this
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //this will understand that the array have at least 5 elements
    println!("The value of a is: {}", a[0]);

    //if the programmer try to acess some index outside of the array
    // the program will panic
    // if in complication time the rust compiler identify a invalid
    // use it will not compile
    // print!("The value of a is: {}", a[10]);
    
}
