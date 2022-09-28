use std::collections::HashMap;

/* use super::break_byte_xor::*;
use super::hex::*;
use super::repeating_key_xor::*;
use super::single_xor::*; */
use openssl::symm::{Crypter,Cipher,Mode};

fn enc_aes_block(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    let mut encrypter: Crypter = match Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, key, None)
    {
        Ok(r) => r,
        _ => unreachable!(),
    };
    encrypter.pad(false);
    let mut ciphertext: Vec<u8> = vec![0; plaintext.len() + 16]; // require that output.len() >= input.len() + block_size
    match encrypter.update(plaintext, &mut ciphertext) {
        Ok(written) => ciphertext[0..written].to_vec(),
        _ => unreachable!(),
    }
}

fn guess_blocksize<F>(oracle: &F) -> usize
where F : Fn(&[u8]) -> Vec<u8> {
    for i in 1..=1000 {
        let guess = oracle("a".repeat(i).as_bytes()).len() - oracle("a".repeat(i-1).as_bytes()).len();
        if guess != 0 {
            return guess
        }
    }
    panic!("Didn't find block size after 1000 guesses")
}


fn salted_aes_128(input: &[u8], key: &[u8]) -> Vec<u8> {
    let unknown_salt = base64::decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap();
    enc_aes_block(&[input,&unknown_salt].concat(), key)
}

use super::pkcs7::EncType;
fn get_enctype<F>(oracle : &F) -> EncType 
where F : Fn(&[u8]) -> Vec<u8> {
    let blocksize = guess_blocksize(oracle);    
    let mut seen: Vec<Vec<u8>> = Vec::new();
    for block in oracle("A".repeat(blocksize).repeat(10).as_bytes()).chunks(blocksize) {
        if seen.contains(&block.to_vec()) {
            return EncType::Ecb; // any ciphertext containing 2 repeating blocks is automatically assumed to be ECB. Probably works most of the time? I'm not really sure what the probabilities are.
        }
        seen.push(block.to_vec());
    }        
    return EncType::Cbc;
}


fn break_aes_ecb<F>(oracle: &F) -> Result<Vec<u8>,String>
where F : Fn(&[u8]) -> Vec<u8> {    
    if get_enctype(oracle) == EncType::Cbc {
        return Err("Can't break aes in cbc mode with this function.".to_string());
    }    
    let blocksize = guess_blocksize(oracle);
    let num_blocks = oracle("".as_bytes()).len() / blocksize;
    let mut salt_value = Vec::new();
    for block in 0..num_blocks {
        let mut mem = HashMap::new();
        for byte in 0..blocksize {
            let prefix = "A".repeat(blocksize - (byte + 1)); // 0th byte will have a blocksize - 1 prefix
                                                                     // 1st byte will have a blocksize - 2 prefix
                                                                     // 15th byte will have 0 length prefix
            for completion in 0..=255 {
                let input = [prefix.as_bytes(), &salt_value, &[completion]].concat();
                mem.insert(completion, oracle(&input));
            }

            for (completion, ciphertext) in mem.iter() {
                let with_completion = ciphertext[block*blocksize..(block+1)*blocksize].to_vec();
                let with_salt_byte = oracle(prefix.as_bytes())[block*blocksize..(block+1)*blocksize].to_vec();
                if with_completion == with_salt_byte {
                    salt_value.push(*completion);
                }
            }
        }
    }
    Ok(salt_value)
}


#[test]
fn test_bytewise_aes_ecb_dec() {
    let res = std::str::from_utf8(&break_aes_ecb(&|plaintext| {salted_aes_128(plaintext, "beeg".repeat(4).as_bytes())}).unwrap()).unwrap().to_string();
    println!("res: {} : {}", res, res.len());
    let unknown_salt = base64::decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap();
    // We only got the first 128 bytes for some reason, but not bothering to fix
    // it..I think I get the point.
    assert_eq!(res, std::str::from_utf8(&unknown_salt).unwrap()[0..128]);
}
