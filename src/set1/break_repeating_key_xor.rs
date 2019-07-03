use bit_vec::BitVec;
use std::convert::TryInto;
use itertools::Itertools;
use super::repeating_key_xor::*;
use super::break_byte_xor::*;

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
    let KEYSIZE: usize = (enc.len() / 2).min(guess.try_into().unwrap());
    let first = String::from_utf8(enc[0..KEYSIZE].to_vec()).unwrap();
    let second = String::from_utf8(enc[KEYSIZE..].to_vec()).unwrap();
    let (ans, _) = hamming(&first, &second).unwrap().overflowing_div(KEYSIZE.try_into().unwrap());
    ans
}

fn transpose(blocks: &[&[u8]]) -> Vec<Vec<u8>> {
    // precondition: all the blocks have the same length, which is the length of the key
    let keysize = blocks.get(0).unwrap().len();
    let mut tr_blocks = Vec::new();
    for i in 0 .. keysize {
        let mut i_block = Vec::new();
        for block in blocks {
            i_block.push(*block.get(i).unwrap());
        }
        tr_blocks.push(i_block);
    }
    tr_blocks
}

fn possible_keys(enc: &[u8]) -> Vec<Vec<u8>> {
    let guesses = (0 ..= 40).map(|k| norm_dist(k, enc)).sorted().take(3);
    let mut keyguesses = Vec::new();
    for guess in guesses {
        // guess is KEYSIZE
        // break the ciphertext into blocks of KEYSIZE len
        // transpose the blocks: make a block of the first byte of every block, the second byte of every block, etc.
        let transposed = transpose(&enc.chunks(guess.try_into().unwrap()).map(|i| i).collect::<Vec<&[u8]>>());
        let mut key = Vec::new();
        for block in transposed {
            let keybyte = find_best_key(&String::from_utf8(block).unwrap().into_bytes());
            key.push(keybyte);
        }
        // solve each transposed block as if single-char XOR
        // for each block, the single-byte xor key producing the best looking histogram is the repeating-key xor key byte for that block
        // put those together and you have the key
        keyguesses.push(key);
    }
    keyguesses
}

#[test]
fn test_hamming() {
    assert_eq!(hamming("this is a test", "wokka wokka!!!").unwrap(),37)
}

#[test]
fn test_possible_keys() {
    assert_eq!(possible_keys(b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272"), vec![vec![0]]);
}
