// Rust uses snake case. This means all naming is underscored and has no capital letters.
// Rusts IO library. We aren't using much of it for now.
// Unlike C you don't need to import every function as a library.
use std::io::{self, Write};
mod encrypt;
use encrypt::encrypt_text;
// This is hoe we colour the text to scare the hackewrs away.
// To quote TechRules, my faveourite YouTuber who hasn't uploaded in 2 years.
// This makes no practical sense. "Haha I have scared the hacker!" What's that going to do?
use colored::Colorize;

fn main() {
    // For some reason those brackets are needed for the colour.
    print!("{}", "Hold on! Are you meant to be here?".red());
    print!("Enter your Passkey.");
    // We should be disabling echo for password entering but how this is done depends on if you are running Windows or Linux.
    // Get user to input passkey.
    let mut passkey_p = String::new();
    io::stdout().flush().unwrap(); // flush it to the screen
    std::io::stdin().read_line(&mut passkey_p).unwrap();
    // This should be a constant but Rust does not allow constants to be casted to by another variable.
    let passkey = passkey_p.trim();
    // Drop passkey_p to avoid it being pulled from memory.
    // Unlike Python Rust does not have an easy way to just kill a value in memory. The best we can so is drop it. And wait for the compiler to kill it.
    let _ = &passkey_p;
    encrypt_text(&passkey);
}