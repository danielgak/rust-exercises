mod front_of_house;

use crate::nested_modules::front_of_house::hosting::remove_from_waitlist;

// brining 2 function to scope and exporting them in the nested_module module
pub use crate::nested_modules::front_of_house::hosting::{more_functions, use_toilet};

// brining io and io::Write
use std::io::{self, Write};
use std::{alloc, cmp::Ordering};

// bringing everyting
use std::collections::*;

/**
 * front_of_house isn’t public, because the eat_at_restaurant function is defined in the same module as front_of_house (nested_modules)
 * (that is, eat_at_restaurant and front_of_house are siblings), we can refer to front_of_house from eat_at_restaurant
 */
pub fn eat_at_restaurant() {
    // Absolute path
    crate::nested_modules::front_of_house::hosting::add_to_waitlist();

    // Relative path (we are in crate::nested_modules)
    front_of_house::hosting::add_to_waitlist();

    // here this one is not aceesible
    // front_of_house::ring_the_door()

    // Idiomatic use
    remove_from_waitlist()
}
