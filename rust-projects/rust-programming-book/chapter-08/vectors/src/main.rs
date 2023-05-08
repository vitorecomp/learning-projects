fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);


    let v = vec![1, 2, 3, 4, 5];

    //the line below will cause a panic
    //this is used when you want to ganrantee that the index you get is valid
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);


    //the interaction in the vector could be done by element
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    //and also by mutable reference
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    //this should be used will cautin maily becuase the vector will waste memory
    //the allowcation will follow the biggest type
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
    //as any element the vector will be freed when it goes out of scope
}


fn invalid_borrow () {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    //is not possible to modify the vector while there is has any reference
    //this reference could be mutable or not

    //the line below will give a compilation error
    // v.push(6);

    //this expose a the implementation of the vector
    //because it will happen maily because is some times
    //necessary to move the vector to another place in memory
    //because the vector could grow and the current position memory could not be enough

    println!("The first element is: {first}");
}