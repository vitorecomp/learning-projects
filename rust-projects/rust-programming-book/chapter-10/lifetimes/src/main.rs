

//this is the annotation of lifetimes, this will allow the
//the compiler to relate the lifetime of the reference to the
//new points, meaning that will define that if one is valid
//their pointer will aslo be valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let r = 2;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        //the line below will present the same error from the line 19,
        //this is given that the string2 is not valid anymore
        //when it can be used
        // result = longest(string1.as_str(), string2.as_str());
        result = "";
    }
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}