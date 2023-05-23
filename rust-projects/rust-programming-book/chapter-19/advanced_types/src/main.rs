//synomns are the type def of c++
//this allow for a smaller way to write code
//avoid the repetion of code

use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

//this can the better written by:
type SimpleResult<T> = std::result::Result<T, std::io::Error>;
pub trait SmallWrite {
    fn write(&mut self, buf: &[u8]) -> SimpleResult<usize>;
    fn flush(&mut self) -> SimpleResult<()>;

    fn write_all(&mut self, buf: &[u8]) -> SimpleResult<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> SimpleResult<()>;
}

enum SimpleOption<T> {
    Some(T),
    None,
}

impl<T> SimpleOption<T> {
    pub fn unwrap(self) -> T {
        match self {
            SimpleOption::Some(val) => val,
            SimpleOption::None => panic!("called `Option::unwrap() on a None value"),
        }
    }
}


fn main() {
    println!("Hello, world!");

    //the idea of use &str is to allow the
    //usage of diferente sized variables, 
    //this is necessary because for rust
    //all the var should the the know size on compile time
    
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";

    
}
