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

#[test]
fn test_single_xor() {
    assert_eq!(single_xor(&vec![0x41,0x41,0x41,0x41,0x41],0), vec![0x41,0x41,0x41,0x41,0x41]);
}
