use bit_vec::BitVec;
use std::convert::TryInto;
use super::rotating_xor::*;

fn diff_bits(a: u8, b: u8) -> u8 {
    let xor_a_b = a ^ b;
    // 0 <= count <= 8 will fit into a u8 so below conversion is justified
    BitVec::from_bytes(&vec![xor_a_b])
        .into_iter()
        .filter(|b| if *b {true} else {false})
        .count().try_into().unwrap()
}

fn hamming(a: &str, b: &str) -> Result<u64, ()> {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    if a_bytes.len() == b_bytes.len() {
        Ok(a_bytes
           .into_iter()
           .zip(b_bytes.into_iter())
           .map(|(ab, bb)| diff_bits(*ab, *bb))
           .map(|x| x.try_into().unwrap())
           .fold(0,|x: u64,y: u64| x + y))
    } else {
        Err(())
    }
}

fn norm_dist(guess: u64, enc: &[u8]) -> u64 {
    let KEYSIZE: usize = (enc.len() / 2).max(guess.try_into().unwrap());
    let first = String::from_utf8(enc[0..KEYSIZE].to_vec()).unwrap();
    let second = String::from_utf8(enc[KEYSIZE..].to_vec()).unwrap();
    let (ans, _) = hamming(&first, &second).unwrap().overflowing_div(KEYSIZE.try_into().unwrap());
    ans
}

fn break_it(enc: &[u8]) -> Vec<u8> {
    let KEYSIZE = (enc.len() / 2).max(40);
    let first = String::from_utf8(enc[0..KEYSIZE].to_vec()).unwrap();
    let second = String::from_utf8(enc[KEYSIZE..].to_vec()).unwrap();
    let d = norm_dist(KEYSIZE, )

}

#[test]
fn test_hamming() {
    assert_eq!(hamming("this is a test", "wokka wokka!!!").unwrap(),37)
}
