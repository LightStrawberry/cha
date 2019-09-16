use std::io;

use crate::App;

pub struct Convert {
    pub st: String,
}

impl Convert {
    pub fn new(st: String) -> Convert {
        println!("{}", 111);
        Convert { st }
    }

    pub fn format_and_print(&self) {
        println!("{}", &self.st);
    }
}
