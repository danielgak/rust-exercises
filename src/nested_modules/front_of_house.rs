// as someone defined the module as front_of_house.rs, now this file behaves as a mod {} definition

fn _ring_the_door() {}

pub mod catering;
pub mod serving;
// front_of_house
//  -> catering (folder)
//  -> serving (folder)
//  -> hosting (pub child expressive)

// also mod can be expresevly defined and nested as in here
pub mod hosting {
    use super::catering;

    pub fn add_to_waitlist() {}

    pub fn remove_from_waitlist() {}

    pub fn _use_toilet() {}

    pub fn _more_functions() {}

    fn _seat_at_table() {
        crate::nested_modules::front_of_house::_ring_the_door(); // absolute path. Child sub modules can access private parent elements
        catering::_serve_order(); // relative path
    }
}
