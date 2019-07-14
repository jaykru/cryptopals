use super::base64;
use super::fixed_xor;

#[derive(Debug)]
pub enum Error {
    DomainErr(String),
    FormatErr(String),
}

use Error::*;

pub fn single_xor(h: &[u8], k: u8) -> Vec<u8> {
    h.into_iter().map(|b| b ^ k).collect::<Vec<u8>>()
}
