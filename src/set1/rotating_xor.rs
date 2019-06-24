use super::single_xor::*;

fn rotating_xor(i: &str, key: &[u8]) -> Result<String,()> {
    let period = key.len();
    let encrypted_bytes: Vec<u8> = i.as_bytes().into_iter().enumerate().map(|(i, b)| b ^ (key.get(i % period)).unwrap()).collect();
    if let Ok(s) = String::from_utf8(encrypted_bytes) {
        Ok(s)
    } else{
        Err(())
    }
}

#[test]
fn test_rotating_xor() {
    if let Ok(v) = single_xor("6a6179", 0x41) {
        if let Ok(s) = String::from_utf8(v) {
            if let Ok(ss) = rotating_xor("jay", &vec![0x41]) {
                assert_eq!(s,ss);
            }
        }
    }
}
