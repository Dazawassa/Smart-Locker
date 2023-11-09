// Rust uses snake case. This means all naming is underscored and has no capital letters.
// Rusts IO library. We aren't using much of it for now.
// Unlike C you don't need to import every function as a library.
use std::io::{self, Write};
mod encrypt;
use encrypt::encrypt_text;
mod pass_check;
use pass_check::check_passkey;
mod selections;
use selections::selections_private_pull;
// This is how we colour the text to scare the hackers away.
// To quote TechRules, my faveourite YouTuber who hasn't uploaded in 2 years.
// This makes no practical sense. "Haha I have scared the hacker!" What's that going to do?
use colored::Colorize;

fn main() {
    // For some reason those brackets are needed for the colour.
    print!("{}", "Hold on! Are you meant to be here?".red());
    print!("Enter your Passkey.");
    // Get user to input passkey.
    // Can't bother to do this for the moment but we should declare how many bits this passkey has for memory efficiency.
    let mut passkey_p = String::new();
    io::stdout().flush().unwrap(); // flush it to the screen
    std::io::stdin().read_line(&mut passkey_p).unwrap();
    // This should be a constant but Rust does not allow constants to be casted to by another variable.
    let passkey = passkey_p.trim();
    // Drop passkey_p to avoid it being pulled from memory.
    // Unlike Python Rust does not have an easy way to just kill a value in memory. The best we can so is drop it. And wait for the compiler to kill it.
    let _ = &passkey_p;
    // This should be fine because the original value is dropped after this is ran.
    // Remember. Trying to beat the compiler isn't possible. But that doesn't mean we shouldn't.
    // Oh yea. What this is doing. Forgot about that one.
    // This calls a fuinction from the encryption model that eventuially will be used to encrypt our text. We then pass the encrypted text into the passkey_encrypted value here.
    let passkey_encrypted = encrypt_text(&passkey);
    // Drop passkey to avoid it being pulled from memory.
    let _ = &passkey;
    // Now we check if the password is valid.
    let password_check = check_passkey(&passkey_encrypted);
    // We do not need the Encrypted password now. So we drop it.
    let _ = &passkey_encrypted;
    // We check if the password was valid. I would rather do this in the module. Even though this looks less clean it runs faster.
    // Because of how this effects data flow we can not drop the check if the password was true or false.
    match password_check {
        true => selections_private_pull(),
        false => println!("Access denied!"),
    }
}