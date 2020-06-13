use colored::*;

pub struct Log;

impl Log {
    pub fn print_ok(text: &str) {
        println!("{}", text.green());
    }

    pub fn print_error(text: &str) {
        println!("{}", text.red());
    }
}