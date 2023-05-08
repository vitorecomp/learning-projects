use std::fs::File;
use std::fs;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    read_file_with_error_treatment();

    //this implementation will just unwrap the error
    //in case off fail a panic will be raised
    let greeting_file = File::open("hello.txt").unwrap();

    //if you want to handle the error you can use expect
    //this will allow to send a especific message on the panic
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn read_file_with_error_treatment() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


fn read_username_from_file_short() -> Result<String, io::Error> {
    //the operator ? will return the error to the caller
    //this allow for a shorter code with less boilerplate
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


fn read_username_from_file_minimal() -> Result<String, io::Error> {
    let mut username = String::new();

    //is necessary to understand that the ? operator is used to return the error
    //so the return type of the function must be the same from the call function
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_liner() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// in case of a function that returns Option on the place of a result,
// this is useful mainly when you don't care about the error that will
// be returned
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}