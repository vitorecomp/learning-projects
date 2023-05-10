

//the command below will force the test
//to run consecutively, if not pass the default
//behavior is to run in parallel
//cargo test -- --test-threads=1


fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    //by default the test that run as as success will not
    //show the output, being showed only when the test fail
    //to overwrite this behavior, use the command below
    //cargo test -- --show-output
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    //the ingore flag will ignore the test
    //this is nice for some cases where you want to debug the code

    //another nice thins is that is possible to run just the
    //ignored test by using the command below
    //cargo test -- --ignored
    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    //in case the cargo test dint find any test
    //with the name it will match every one will the
    //name prefix
    //cargo test add -> will run all the test with the name add
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    //is possible to run a specific test
    //by using the follow command:
    //cargo test one_hundred
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}


pub fn add_two(a: i32) -> i32 {
    a + 2
}


fn main() {
    println!("Hello, world!");
}
