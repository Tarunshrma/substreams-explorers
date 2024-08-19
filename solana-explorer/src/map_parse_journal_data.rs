use crate::pb::sol::transactions::{journal::v1::{JournalEntry, Journals}, v1::Instructions};


use std::error::Error;

#[substreams::handlers::map]
fn map_parse_instruction_data(insts: Instructions) -> Result<Journals, substreams::errors::Error> {
    let transaction_data: Vec<JournalEntry> = insts.instructions.iter()
        .map(|inst| {
            // Assuming inst.data contains the Base58 encoded string
            match decode_and_parse_to_protobuf(inst.data.clone()) {
                Ok(parsed_message) => {
                    JournalEntry {
                        // owner: parsed_message.owner,
                        title: parsed_message.title, // Replace field1 with the actual field name
                        message: parsed_message.message, // Replace field2 with the actual field name
                    }
                },
                Err(e) => {
                    // Handle the error appropriately; here we just return a default value
                    JournalEntry {
                        // owner: String::from("Error").into(),
                        title: String::from("Error"),
                        message: format!("Failed to parse: {}", e),
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    Ok(Journals { journals: transaction_data })
}


fn decode_and_parse_to_protobuf(data: Vec<u8>) -> Result<JournalEntry, Box<dyn Error>> {

    substreams::log::info!("raw data: {:?}", data);


    // let decoded_bytes = Vec::from_hex(&data[8..])?;
    let skipped_bytes = &data[8..];

    substreams::log::info!("Skipped bytes: {:?}", skipped_bytes);

    // Step 2: Deserialize the byte array into the Protobuf structure
    //let message = TransactionData::decode(&*bytes_to_parse)?;
// Function to read a length-prefixed string
    fn read_length_prefixed_string(data: &mut &[u8]) -> Result<String, Box<dyn Error>> {
        // Read the length of the string (4 bytes little-endian)
        if data.len() < 4 {
            return Err("Data too short to read length".into());
        }
        let length = u32::from_le_bytes(data[..4].try_into()?) as usize;
        *data = &data[4..];

        // Ensure data is large enough for the string
        if length > data.len() {
            return Err("Data too short for string".into());
        }

        // Extract the string
        let string_bytes = &data[..length];
        let result = String::from_utf8(string_bytes.to_vec())?;
        *data = &data[length..];
        
        Ok(result)
    }

    let mut dt = skipped_bytes;
    let title = read_length_prefixed_string(&mut dt)?;
    let msg: String = read_length_prefixed_string(&mut dt)?;

    substreams::log::info!("Parsed title and message : {:?} {:?}", title ,msg);


    //let message = TransactionData::decode(&*skipped_bytes)?;
    let message = JournalEntry{
        title: title,
        message: msg,
    };

    substreams::log::info!("Parsed message : {:?}", message);

    Ok(message)
}