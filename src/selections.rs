use std::io::{self, Write};

// This function exists for security reasons. It obfuscates data transfer in practise and avoids attacks that work from public values.
pub fn selections_private_pull(){
    selections_main();
}

// This function we actually want.
fn selections_main(){
    print!("Welcome to the RFID cloning program!\n");
    // Sorry Adam guess you can break infinite loops. But you can't break my heart.
    loop {
        // Greeting the user and then aksing for our input.
        // We should store statistics for each of these functions. Then we could write an R program which can generate bar graphs and pie charts.
        print!("Please select which service would be required\n Generate - Genrate a new RFID value to be flashed.\n Restore - Restore an old value onto a card\n Admin - Flash admin value that can access any locker\n Format - Reset code on a locker\n Quit - Exit the program\n");
        let mut choice = String::new();
        io::stdout().flush().unwrap(); // flush it to the screen

        io::stdin().read_line(&mut choice).unwrap();
        // Convert to lowercase. This avoid the user having to type in the exact case.
        // If someone can make this a single statment I would be thanklful. You definetly can but I kept crashing when I did.
        let choice = choice.to_lowercase();
        let choice = choice.trim(); // remove trailing newline
        // I know processing the users input in a sperate function is slower. But if we intend to use a GUI later on this will make that easier.
        // We are passing choice as a pointer. Pointers do not exist in Python but are very common in C. Learn pointers else you will suffer the pain of a thousand suns.
        user_selection(&choice);
    }
}

fn user_selection(choice: &str) {
    match choice {
        "quit" | "exit" => std::process::exit(0),
        "generate" => print!("This is the generate function.\n"),
        _ => println!("Please enter a valid choice for this program."),
    }
}