

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

//this is a really important flag to make sure that the compiler 
//doesn't try to include it on the final build
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

