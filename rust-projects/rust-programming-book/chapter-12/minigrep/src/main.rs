//to run this code is necessary to use the follow
//cargo command
//cargo run -- searchstring example-filename.txt


use std::env;
use std::fs;
use std::process;

use minigrep::Config;



fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    println!("------------------");
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    println!("In file {}", file_path);


    println!("------------------");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    println!("------------------");
    //read the file
    //if the file not exists, the program will panic
    let config = Config::build(&args).expect("Invalid arguments");
    let lines = minigrep::run(config).expect("Error on the program");
    dbg!(lines);
    
    
    println!("------------------");
    let args: Vec<String> = env::args().collect();

    let new_config = Config::build(&args).expect("Problem parsing arguments: {err}");

    if let Err(e) = minigrep::run(new_config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}



