
mod hello_trait;

use macros::HelloMacro;
use hello_trait::HelloMacro;


#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}