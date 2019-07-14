use super::base64::*;
use super::single_xor::*;

fn repeating_key_xor(i: &[u8], key: &[u8]) -> Vec<u8> {
    let period = key.len();
    let encrypted_bytes: Vec<u8> = i.into_iter().enumerate().map(|(i, b)| b ^ (key.get(i % period)).unwrap()).collect();
    encrypted_bytes
}

#[test]
fn test_repeating_key_xor() {
    let v = single_xor(&hex_as_bytes("6a6179").unwrap(),0x41); 
    if let Ok(s) = String::from_utf8(v) {
        let ss = repeating_key_xor(&vec![b'j',b'a',b'y'], &vec![0x41]);
        assert_eq!(s.as_bytes().to_vec(),ss);
    }
}
