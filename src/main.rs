mod models;

use std::fs;
use models::coin_bank::CoinBank;

const COIN_BANK_FILENAME: &str = "coin_bank.json";

fn main() {
    let coin_bank = read_coin_bank(COIN_BANK_FILENAME);

    let (dollars, cents) = coin_bank.dollars_and_cents();
    println!("Initial coin_bank in dollars and cents: ${}.{}", dollars, cents);

    // Add money to the coin bank
    let coin_bank = coin_bank.add_quarters(4);

    let (dollars, cents) = coin_bank.dollars_and_cents();
    println!("Updated coin_bank: ${}.{}", dollars, cents);
}

fn read_coin_bank(filename: &str) -> CoinBank {
    // Read the current state of the coin bank from a file
    let coin_json_content = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    serde_json::from_str(&coin_json_content)
        .expect("Read string is not properly formed JSON")
}
