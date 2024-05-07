// This file is for writting the cards.
use std::fs::File;
use csv::Reader;
use std::error::Error;

pub fn write_card(locker_id: &str) -> Result<String, Box<dyn Error>> {
    let cards_list = "src/cards/card_list.csv";
    let card_result = card_read(&cards_list, &locker_id)?;

    // You could process or simply return the result from card_read
    Ok(card_result)
}

fn card_read(_cards_list: &str, _locker_id: &str) -> Result<String, Box<dyn Error>> {
    // This works for returning a dummy value.
    //Ok("Valid card processed".to_string())
}
