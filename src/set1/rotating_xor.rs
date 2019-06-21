use super::single_xor::*;
use std::convert::TryInto;

fn rotating_xor(i: &str, key: &[u8]) -> Result<String,()> {
    let period = key.len();
    let encrypted_bytes: Vec<u8> = i.as_bytes().into_iter().enumerate().map(|(i, b)| b ^ (key.get(i % period)).unwrap()).collect();
    if let Ok(s) = String::from_utf8(encrypted_bytes) {
        Ok(s)
    } else{
        Err(())
    }
}

use bit_vec::BitVec;
fn diff_bits(a: u8, b: u8) -> u8 {
    let xor_a_b = a ^ b;
    // 0 <= count <= 8 will fit into a u8 so below conversion is justified
    BitVec::from_bytes(&vec![xor_a_b]).into_iter().filter(|b| if *b {true} else {false}).count().try_into().unwrap()
}
fn hamming(a: &str, b: &str) -> Result<u64, ()> {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    if a_bytes.len() == b_bytes.len() {
        Ok(a_bytes.into_iter().zip(b_bytes.into_iter()).map(|(ab, bb)| diff_bits(*ab, *bb)).map(|x| x.try_into().unwrap()).fold(0,|x: u64,y: u64| x + y))
    } else {
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

    assert_eq!("false", rotating_xor("jay", &vec![0x69,0x42,0x69]).unwrap())
}

#[test]
fn test_hamming() {
    assert_eq!(hamming("this is a test", "wokka wokka!!!").unwrap(),37)
}