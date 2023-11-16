use std::{error::Error, process, io::{self, Write}};
use std::fs::File;
use csv::Reader;

pub fn check_passkey(passkey_encrypted: &str) -> Result<bool, Box<dyn Error>> {
    let password_list = "src/accounts/passwords.csv"; // Replace this with your file path
    let password_matched = password_read(password_list, passkey_encrypted)?;
    
    Ok(password_matched)
}

fn password_read(password_list: &str, entered_password: &str) -> Result<bool, Box<dyn Error>> {
    let file = File::open(password_list)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut password_matched = false;

    for result in rdr.records() {
        let record = result?;
        if let Some(password) = record.get(0) {
            if password == entered_password {
                password_matched = true;
                break;
            }
        }
    }

    if password_matched {
        Ok(true)
    } else {
        Ok(false)
    }
}