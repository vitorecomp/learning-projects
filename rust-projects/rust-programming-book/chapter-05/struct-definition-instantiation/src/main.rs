struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64,
}


//tuple struct (allow for the creation of struct where the name of the parameters are not important)
struct Color(i32, i32, i32);


//unit struct (allow for the creation for the use with traits)
struct UnitStruct;

fn main() {
    
    let mut user = User {
        active: true,
        name: String::from("John"),
        email: String::from("vitor.ecomp@gmail.com"),
        sign_in_count: 0,
    };
    
    println!("User active: {}", user.active);
    println!("User name: {}", user.name);
    println!("User email: {}", user.email);
    println!("User sign_in_count: {}", user.sign_in_count);

    //is also possible to make a mutable struct
    //to make it possible the entire struct should be mutable
    //meaning that is not possible to have just one field mutable
    user.name = String::from("Vitor");

    let user2 = build_user(String::from("vitor.ecomp@gmail.com"), String::from("vitor"));
    
    println!("User2 active: {}", user2.active);
    println!("User2 name: {}", user2.name);
    println!("User2 email: {}", user2.email);
    println!("User2 sign_in_count: {}", user2.sign_in_count);

    //the rust all make possible the usage of the destructuring feature
    //this will allow fot a better way to create a new struct based on another one

    // important!!
    //this feature will move any value that is not a primitive type
    //this make the last struct unusable

    let user3 = User {
        name: String::from("Vitor Araujo"),
        ..user2
    };
    println!("User3 active: {}", user3.active);
    println!("User3 name: {}", user3.name);
    println!("User3 email: {}", user3.email);
    println!("User3 sign_in_count: {}", user3.sign_in_count);
    
    let color_black = Color(0, 0, 0);
    println!("Color black: {}, {}, {}", color_black.0, color_black.1, color_black.2);

    let unit_struct = UnitStruct;

}


fn build_user(email: String, name: String) -> User {
    //a nice quality of life feature is that if the parameter name is the same as the struct field name
    //is not necessary to repeat the name
    User {
        active: true,
        name,
        email,
        sign_in_count: 1,
    }
}