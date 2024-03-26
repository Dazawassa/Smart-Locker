// Rust uses snake case. This means all naming is underscored and has no capital letters.
// Rusts IO library. We aren't using much of it for now.
// Unlike C you don't need to import every function as a library.
use std::{error::Error, process, io::{self, Write}};
mod encrypt;
use encrypt::encrypt_text;
mod pass_check;
use pass_check::check_passkey;
mod selections;
use selections::selections_private_pull;
// This is how we colour the text.
use colored::Colorize;

fn main() {
    // Asking for a user password
    print!("{}", "Hold on! Are you meant to be here?\n".red());
    print!("Enter your Passkey.\n");
    // Get user to input passkey.
    let mut passkey_p = String::new();
    io::stdout().flush().unwrap(); // flush it to the screen
    std::io::stdin().read_line(&mut passkey_p).unwrap();
    let passkey = passkey_p.trim();
    // Drop passkey_p to avoid it being pulled from memory.
    // Unlike Python Rust does not have an easy way to just kill a value in memory. The best we can so is drop it. And wait for the compiler to kill it.
    let _ = &passkey_p;
    // This should be fine because the original value is dropped after this is ran.
    let passkey_encrypted = encrypt_text(&passkey);
    let _ = &passkey;
    match check_passkey(&passkey_encrypted) {
        Ok(true) => {
            let _ = &passkey_encrypted;
            selections_private_pull();
        }
        Ok(false) => {
            println!("Access denied!");
        }
        // Handle error.
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
