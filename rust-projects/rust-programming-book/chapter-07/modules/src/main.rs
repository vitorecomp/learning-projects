
//this file is normally the entry point
//for any binary crate

//is also possible to import other modules
//with the mod keyword
//mod my_module;
//this will make the compiler look for a file
//named my_module.rs or my_module/mod.rs




//is also possible to import create submodules
//this will be created inside the module folder
//and they will work the same way as the main module



pub mod my_module;

use crate::my_module::my_submodule::PakcageEnum;



fn main() {
    let plant = PakcageEnum::A;
    println!("I'm growing {:?}!", plant);
}
