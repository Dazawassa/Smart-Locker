// Rust uses snake case. This means all naming is underscored and has no capital letters.
// Rusts IO library. We aren't using much of it for now.
// Unlike C you don't need to import every function as a library.
use std::{error::Error, process, io::{self, Write}};
mod encrypt;
use encrypt::encrypt_text;
// This is hoe we colour the text to scare the hackewrs away.
// To quote TechRules, my faveourite YouTuber who hasn't uploaded in 2 years.
// This makes no practical sense. "Haha I have scared the hacker!" What's that going to do?
use colored::Colorize;

fn main() {
    // For some reason those brackets are needed for the colour.
<<<<<<< Updated upstream
    print!("{}", "Hold on! Are you meant to be here?".red());
    print!("Enter your Passkey.");
    // We should be disabling echo for password entering but how this is done depends on if you are running Windows or Linux.
=======
    print!("{}", "Hold on! Are you meant to be here?\n".red());
    print!("Enter your Passkey.\n");
>>>>>>> Stashed changes
    // Get user to input passkey.
    let mut passkey_p = String::new();
    io::stdout().flush().unwrap(); // flush it to the screen
    std::io::stdin().read_line(&mut passkey_p).unwrap();
    // This should be a constant but Rust does not allow constants to be casted to by another variable.
    let passkey = passkey_p.trim();
    // Drop passkey_p to avoid it being pulled from memory.
    // Unlike Python Rust does not have an easy way to just kill a value in memory. The best we can so is drop it. And wait for the compiler to kill it.
    let _ = &passkey_p;
<<<<<<< Updated upstream
    encrypt_text(&passkey);
=======
    // This should be fine because the original value is dropped after this is ran.
    // Remember. Trying to beat the compiler isn't possible. But that doesn't mean we shouldn't.
    // Oh yea. What this is doing. Forgot about that one.
    // This calls a fuinction from the encryption model that eventuially will be used to encrypt our text. We then pass the encrypted text into the passkey_encrypted value here.
    let passkey_encrypted = encrypt_text(&passkey);
    // Drop passkey to avoid it being pulled from memory.
    let _ = &passkey;
    // Now we check if the password is valid.
    match check_passkey(&passkey_encrypted) {
        Ok(true) => {
            println!("Access granted!");
            // We null the entered passkey so it can't be pulled from memory.
            let _ = &passkey_encrypted;
            selections_private_pull();
            // Continue with further operations
        }
        Ok(false) => {
            // Nice hustle tons of fun.
            println!("Access denied!");
        }
        // Handle error.
        Err(err) => {
            // Error occurred
            println!("Error: {}", err);
            // Handle error case
        }
    }
>>>>>>> Stashed changes
}