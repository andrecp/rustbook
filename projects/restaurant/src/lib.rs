// Module organizes code inside a crate.

mod front_of_house {
    // private by default outside the module
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// pub makes it public!
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path, front_of_house is defined at same level as eat_at_restaurant
    // so public!
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super allows to go up in the tree.
        super::serve_order();
    }

    fn cook_order() {}
}
