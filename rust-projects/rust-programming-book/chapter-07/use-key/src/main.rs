use std::collections::HashMap;

use std::fmt;
use std::io;

use std::io::Result as IoResult;

// use std::{cmp::Ordering, io};
// use std::io;
// use std::io::Write;
// use std::collections::*;

fn function1() -> fmt::Result {
    // --snip--
    fmt::Result::Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    io::Result::Ok(())
}

fn function3() -> IoResult<()> {
    // --snip--
    io::Result::Ok(())
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}