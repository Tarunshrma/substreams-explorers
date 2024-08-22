use std::{convert::TryInto, error::Error};
use crate::{pb::sol::transactions::journal::v1::JournalEntry, utils::constants::DISCRIMINATOR};

use super::decode_data::DecodeData;

impl DecodeData for JournalEntry {
    fn parse_from_data(data: &[u8], hash_of_accounts:String) -> Result<Self, Box<dyn Error>> {
        // Skip the first 8 bytes
        let skipped_bytes = &data[DISCRIMINATOR..];

        // Function to read a length-prefixed string
        fn read_length_prefixed_string(data: &mut &[u8]) -> Result<String, Box<dyn Error>> {
            if data.len() < 4 {
                return Err("Data too short to read length".into());
            }
            let length = u32::from_le_bytes(data[..4].try_into()?) as usize;
            *data = &data[4..];

            if length > data.len() {
                return Err("Data too short for string".into());
            }

            let string_bytes = &data[..length];
            let result = String::from_utf8(string_bytes.to_vec())?;
            *data = &data[length..];

            Ok(result)
        }

        let mut dt = skipped_bytes;
        let title = read_length_prefixed_string(&mut dt)?;
        let msg = read_length_prefixed_string(&mut dt)?;

        let id = format!("{}-{}", title, hash_of_accounts);

        Ok(JournalEntry { id,  title, message: msg })
    }
}
