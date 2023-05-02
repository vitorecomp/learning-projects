fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("The first word is: {}", word);

    //will be impossible to invalidate the string
    // mainly because the reference is still in use
    // s.clear();
    println!("The first word is: {}", word);

    //the slices can be done in any type that can be indexed
    //this will allow for a generic use of them

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
