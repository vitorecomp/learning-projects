mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}


use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant_simple() {
    add_to_waitlist();
}