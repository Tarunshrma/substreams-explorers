use crate::pb::sol::transactions::{journal::v1::{JournalEntry, Journals}, v1::Instructions};
use crate::parsers::decode_data::DecodeData;
use std::error::Error;

#[substreams::handlers::map]
fn map_parse_journal_data(insts: Instructions) -> Result<Journals, substreams::errors::Error> {
    let transaction_data: Vec<JournalEntry> = insts.instructions.iter()
        .map(|inst| {
            // Assuming inst.data contains the Base58 encoded string
            match decode_and_parse_to_protobuf::<JournalEntry>(inst.data.clone()) {
                Ok(parsed_message) => {
                    parsed_message
                },
                Err(e) => {
                    // Handle the error appropriately; here we just return a default value
                    JournalEntry {
                        title: String::from("Error"),
                        message: format!("Failed to parse: {}", e),
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    Ok(Journals { journals: transaction_data })
}

fn decode_and_parse_to_protobuf<T: DecodeData>(data: Vec<u8>) -> Result<T, Box<dyn Error>> {
    T::parse_from_data(&data)
}