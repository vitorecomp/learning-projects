
//is possible to set the variable of BACK_TRACE
//to get the stack form any panic, this will allow for a better error debug
//and also a better understanding of the code
//RUST_BACKTRACE=1 cargo run

//this will just work no in the release mode
fn main() {
    panic!("crash and burn");
}
