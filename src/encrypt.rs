// This should be private but we just need this to work right now.
pub fn encrypt_text(passkey: &str){
    // This is where we would encrypt our text.
    // Print the passkey to the screen.
    println!("Your passkey is {}", passkey);
    let passkey_encrypted = passkey;

    match passkey_encrypted {
        "quit" | "exit" => std::process::exit(0),
        _ => println!("Invalid passkey."),
    }
}