use colored::*;

/// `Unit struct` that provides log related functions
/// 
/// Example:
/// 
/// Log::print_ok(format!("Hello, {}!", "rust"))
/// Log::print_error(format!("Shields are at {}%", 10))
pub struct Log;

impl Log {
    /// Prints `text` with green foreground
    pub fn print_ok(text: String) {
        println!("{}", text.green());
    }

    /// Prints `text` with red foreground
    pub fn print_error(text: String) {
        println!("{}", text.red());
    }
}