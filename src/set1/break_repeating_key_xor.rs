use super::break_byte_xor::*;
use super::hex::*;
use super::repeating_key_xor::*;
use super::single_xor::*;
use bit_vec::BitVec;
use itertools::Itertools;
use std::convert::TryInto;

fn diff_bits(a: u8, b: u8) -> u8 {
    let xor_a_b = a ^ b;
    // 0 <= count <= 8 will fit into a u8 so below conversion is justified
    BitVec::from_bytes(&vec![xor_a_b])
        .into_iter()
        .filter(|b| if *b { true } else { false })
        .count()
        .try_into()
        .unwrap()
}

fn hamming(a: &[u8], b: &[u8]) -> Result<u64, ()> {
    if a.len() == b.len() {
        Ok(a.into_iter()
            .zip(b.into_iter())
            .map(|(ab, bb)| diff_bits(*ab, *bb))
            .map(|x| x.try_into().unwrap())
            .fold(0, |x: u64, y: u64| x + y))
    } else {
        println!(
            "lengths for hamming not same!\na: {}\n b: {}",
            a.len(),
            b.len()
        );
        Err(())
    }
}

#[test]
fn test_hamming() {
    assert_eq!(
        hamming("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()).unwrap(),
        37
    )
}

fn norm_dist(guess: usize, enc: &[u8]) -> u64 {
    let keysize: usize = guess;
    println!("keysize: {}", keysize);
    let first = &enc[0..keysize];
    let second = &enc[keysize..2 * keysize];
    let hamming_ans = hamming(&first, &second).unwrap();
    println!("unnormed hamming distance: {}", hamming_ans);
    let (normed, _) = hamming_ans.overflowing_div(keysize.try_into().unwrap());
    normed
}

fn transpose(blocks: &[&[u8]]) -> Vec<Vec<u8>> {
    // precondition: all the blocks have the same length, which is the length of the key
    let keysize = blocks.get(0).unwrap().len();
    let mut tr_blocks = Vec::new();
    for i in 0..keysize {
        println!("i: {} keysize: {}", i, keysize);
        let mut i_block = Vec::new();
        for block in blocks {
            println!("block len: {}", block.len());
            i_block.push(*block.get(i).unwrap());
        }
        tr_blocks.push(i_block);
    }
    tr_blocks
}

fn possible_keys(enc: &[u8]) -> Vec<Vec<u8>> {
    let half = enc.len() / 2;
    let guesses = (1..=half)
        .map(|k| norm_dist(k, enc))
        .zip(1..=half)
        .sorted_by(|(norm, _), (norm2, _)| Ord::cmp(norm2, norm))
        .map(|(_, x)| x)
        .filter(|x| enc.len() % x == 0)
        .take(3)
        .collect::<Vec<usize>>();
    println!("length guesses: {:?}", guesses);
    let mut keyguesses = Vec::new();
    for guess in guesses {
        assert!(guess <= half);
        println!("assert succeeded");
        // guess is KEYSIZE
        // break the ciphertext into blocks of KEYSIZE len
        // transpose the blocks: make a block of the first byte of every block, the second byte of every block, etc.
        let transposed = transpose(&enc.chunks(guess).collect::<Vec<&[u8]>>());
        let mut key = Vec::new();
        for block in transposed {
            let keybyte = find_best_key(&block).unwrap();
            key.push(keybyte);
        }
        // solve each transposed block as if single-char XOR
        // for each block, the single-byte xor key producing the best looking histogram is the repeating-key xor key byte for that block
        // put those together and you have the key
        keyguesses.push(key);
        println!("moving to next guess");
    }
    keyguesses
}

#[test]
fn test_possible_keys() {
    let poss = possible_keys(
        &hex_as_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
            .unwrap(),
    );
    assert_eq!(
        "Cooking MC\'s like a pound of bacon",
        String::from_utf8(single_xor(
            &hex_as_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap(),
            *poss.get(0).unwrap().get(0).unwrap(),
        ))
        .unwrap()
    );
}
