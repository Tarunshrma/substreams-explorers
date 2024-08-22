use crate::pb::sol::transactions::{journal::v1::{JournalEntry, Journals}, v1::Instructions};
use crate::parsers::decode_data::DecodeData;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::error::Error;

#[substreams::handlers::map]
fn map_parse_journal_data(insts: Instructions) -> Result<Journals, substreams::errors::Error> {
    let transaction_data: Vec<JournalEntry> = insts.instructions.iter()
        .map(|inst| {
            // Assuming inst.data contains the Base58 encoded string
            let hash_of_accounts = create_unique_hash_from_accounts(inst.accounts.clone()); 
            match decode_and_parse_to_protobuf::<JournalEntry>(inst.data.clone(),hash_of_accounts) {
                Ok(parsed_message) => {
                    parsed_message
                },
                Err(e) => {
                    // Handle the error appropriately; here we just return a default value
                    JournalEntry {
                        id: String::from("-1"),
                        title: String::from("Error"),
                        message: format!("Failed to parse: {}", e),
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    Ok(Journals { journals: transaction_data })
}

fn create_unique_hash_from_accounts(accounts: Vec<String>) -> String {
    let concatenated_accounts: String = accounts.iter()
    .map(|acct| acct.to_string()) // Convert each account ID to a string
    .collect::<Vec<String>>()
    .join(""); // Join all strings without any separator

    // Compute the hash of the concatenated string
    let mut hasher: DefaultHasher = DefaultHasher::new();
    concatenated_accounts.hash(&mut hasher);
    let hash = hasher.finish();
    return hash.to_string();
}

fn decode_and_parse_to_protobuf<T: DecodeData>(data: Vec<u8>, hash_of_accounts:String) -> Result<T, Box<dyn Error>> {
    T::parse_from_data(&data, hash_of_accounts)
}