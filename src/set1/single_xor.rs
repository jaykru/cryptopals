use super::base64;

#[derive(Debug)]
pub enum Error {
    DomainErr(String),
    FormatErr(String),
}

use Error::*;

pub fn single_xor(h: &str, k: u8) -> Result<Vec<u8>, Error> {
    if let Some(b) = base64::hex_as_bytes(String::from(h)) {
        let out_bytes = b.into_iter().map(|bp| bp ^ k).collect::<Vec<u8>>();
        Ok(out_bytes)
    } else {
        Err(FormatErr("Wrong hex format.".to_string()))
    }
}
