use std::slice;

//is possible to create a unsafe function, this one is encapsulated in a
//unsafe block, but just can be called in a unsafe block
unsafe fn dangerous() {}

//this will point for a external library, in this case the C standard library
//this function will be necessary to be called from a unsafe block
//because rust does not have a way of check-in it and guarantee the memory usage.
extern "C" {
    fn abs(input: i32) -> i32;
}

//this is a example of code that will encapsulate a unsafe block
//this will allow two mutable pointer for the save vector, as well
//to execute unsafe memory operations
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}


fn main() {
    let mut num = 5;

    //create a unsage pointer
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;


    //to make use o f a unsafe pointer is necessary
    //to do so in a unsafe block, in this block 
    //the usage of unsafe code allow for this 
    //such as multiple mut pointer, as well as
    //the borrow checker is permissive just running
    //on normal pointers
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    //calling a unsafe function
    unsafe{
        dangerous();
    }


    //if the unsafe function is called outside a unsafe block
    //the compiler will throw a error



    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (v1, v2) = split_at_mut(&mut v, 3);


    //calling the external function abs from C standard library
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }


    //global variables  - Static variables
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

//bonus
//this is a way to let a function to be called from another languages such as C
//this means that the function will be callable from C
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
