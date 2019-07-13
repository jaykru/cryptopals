use super::base64;

#[derive(Debug)]
pub enum Error {
    DomainErr(String),
    FormatErr(String),
}

use Error::*;

pub fn single_xor(h: &[u8], k: u8) -> Vec<u8> {
    let out_bytes = h.into_iter().map(|bp| bp ^ k).collect::<Vec<u8>>();
    out_bytes
}
