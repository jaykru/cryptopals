use super::base64;

#[derive(Debug)]
enum Error {
    DomainErr(String),
    FormatErr(String),
}

use Error::*;

fn single_xor(h: String, k: u8) -> Result<String, Error> {
    if let Some(b) = base64::hex_as_bytes(h) {
        let out_bytes = b.into_iter().map(|bp| bp ^ k).collect::<Vec<u8>>();
        let mut acc = String::from("");
        for byte in out_bytes {
            acc.push_str(&format!("{}", byte));
        }
        Ok(acc.to_string())
    }
    else {
        Err(FormatErr("Wrong hex format.".to_string()))
    }
}

#[test]
fn test_single_xor() {
    if let Ok(s) = single_xor("".to_string(),"686974207468652062756c6c277320657965".to_string()) {
        println!("{}", s);
        assert_eq!(s, "746865206b696420646f6e277420706c6179".to_string());
    }
}
