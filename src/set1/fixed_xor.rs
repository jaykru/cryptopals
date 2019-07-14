use super::hex::*;

#[derive(Debug)]
enum Error {
    DomainErr(String),
    FormatErr(String),
}

use Error::{DomainErr, FormatErr};

fn fixed_xor(h1: &str, h2: &str) -> Result<String, Error> {
    if h1.len() != h2.len() {
        return Err(DomainErr("Need equal number of bytes for each argument to perform fixed-width xor.".to_string()))
    }
    
    if let (Some(h1_b), Some(h2_b)) = (hex_as_bytes(h1), hex_as_bytes(h2)) {
        let out_bytes = h1_b.into_iter().zip(h2_b.into_iter()).map(|(b1, b2)| b1 ^ b2).collect::<Vec<u8>>();
        let mut acc = String::from("");
        for byte in out_bytes {
            acc.push_str(&format!("{:x}", byte));
        }
        Ok(acc.to_string())
    }
    else {
        Err(FormatErr("Wrong hex format.".to_string()))
    }
}

#[test]
fn test_fixed_xor() {
    if let Ok(s) = fixed_xor("1c0111001f010100061a024b53535009181c","686974207468652062756c6c277320657965") {
        println!("{}", s);
        assert_eq!(s, "746865206b696420646f6e277420706c6179".to_string());
    }
}
