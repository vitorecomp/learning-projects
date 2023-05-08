fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();


    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("hello");
    //the code below will break, given the way rust treat strings
    // let h = s1[0];

    for c in "Зд".chars() {
        println!("{c}");
    }
}
