use std::fmt;
use std::io;


// use std::fmt::Result;
// use std::io::Result as IoResult;


// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io};


// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// use std::collections::*;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// re-exporting
pub use crate::front_of_house::hosting;

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}


// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}
