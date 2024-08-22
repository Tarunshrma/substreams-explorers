use std::error::Error;

// Define a trait for parsing data
pub trait DecodeData: Sized {
    fn parse_from_data(data: &[u8], hash_of_accounts:String) -> Result<Self, Box<dyn Error>>;
}