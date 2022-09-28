use openssl::symm::{Cipher, Crypter, Mode};
use rand::prelude::*;
use std::convert::TryInto;

fn pkcs_pad(block: &[u8], k: usize) -> Vec<u8> {
    let l = block.len();
    let diff = k - (l % k);
    let mut block_vec = block.to_vec();
    block_vec.append(&mut vec![
        diff.try_into().unwrap();
        diff.try_into().unwrap()
    ]);
    block_vec
}

#[test]
fn test_pkcs_pad() {
    assert_eq!(
        pkcs_pad(b"YELLOW SUBMARINE", 20),
        b"YELLOW SUBMARINE\x04\x04\x04\x04"
    );
}

// todo: move to set 1
#[derive(Debug)]
enum Error {
    DomainErr(String),
    FormatErr(String),
}

use Error::{DomainErr, FormatErr};

fn block_xor(h1: &[u8], h2: &[u8]) -> Result<Vec<u8>, Error> {
    if h1.len() != h2.len() {
        return Err(DomainErr(
            "Need equal number of bytes for each argument to perform fixed-width xor.".to_string(),
        ));
    }

    let out_bytes = h1
        .into_iter()
        .zip(h2.into_iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect::<Vec<u8>>();
    Ok(out_bytes)
}

fn cbc_enc<F>(plaintext: &[u8], iv: &[u8], block_cipher_enc: F) -> Vec<u8>
where
    F: Fn(&[u8]) -> Vec<u8>,
{
    let padded = pkcs_pad(&plaintext, iv.len()); // pad blocks out to iv length
    let mut ptchunks = padded.chunks(iv.len());
    let mut prev = iv.to_vec();
    let mut ciphertext = Vec::new();
    for block in ptchunks {
        let cipher_input = block_xor(block, &prev).unwrap();
        prev = block_cipher_enc(&cipher_input);
        ciphertext.extend_from_slice(&mut prev);
    }
    return ciphertext;
}

fn cbc_dec<F>(ciphertext: &[u8], iv: &[u8], block_cipher_dec: F) -> Vec<u8>
where
    F: Fn(&[u8]) -> Vec<u8>,
{
    let mut ctchunks = ciphertext.chunks(iv.len());
    let mut prev = iv;
    let mut plaintext = Vec::new();
    for block in ctchunks {
        let mut block_plaintext = block_xor(&block_cipher_dec(block), prev).unwrap();
        plaintext.append(&mut block_plaintext);
        prev = block;
    }
    return plaintext;
}

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

fn dec_aes_block(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let mut decrypter: Crypter = match Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key, None)
    {
        Ok(r) => r,
        _ => unreachable!(),
    };
    decrypter.pad(false);
    let mut plaintext: Vec<u8> = vec![0; ciphertext.len() + 16]; // require that output.len() >= input.len() + block_size
    match decrypter.update(ciphertext, &mut plaintext) {
        Ok(written) => plaintext[0..written].to_vec(),
        _ => unreachable!(),
    }
}

#[test]
fn test_cbc() {
    let test_string = "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";

    let expected = "I\'m back and I\'m ringin\' the bell \nA rockin\' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that\'s my DJ Deshay cuttin\' all them Z\'s \nHittin\' hard and the girlies goin\' crazy \nVanilla\'s on the mike, man I\'m not lazy. \n\nI\'m lettin\' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse\'s to the side yellin\', Go Vanilla Go! \n\nSmooth \'cause that\'s the way I will be \nAnd if you don\'t give a damn, then \nWhy you starin\' at me \nSo get off \'cause I control the stage \nThere\'s no dissin\' allowed \nI\'m in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n\' play \n\nStage 2 -- Yea the one ya\' wanna listen to \nIt\'s off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI\'m an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI\'m like Samson -- Samson to Delilah \nThere\'s no denyin\', You can try to hang \nBut you\'ll keep tryin\' to get my style \nOver and over, practice makes perfect \nBut not if you\'re a loafer. \n\nYou\'ll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I\'m comin\' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin\' \nVanilla Ice is sellin\' and you people are buyin\' \n\'Cause why the freaks are jockin\' like Crazy Glue \nMovin\' and groovin\' trying to sing along \nAll through the ghetto groovin\' this here song \nNow you\'re amazed by the VIP posse. \n\nSteppin\' so hard like a German Nazi \nStartled by the bases hittin\' ground \nThere\'s no trippin\' on mine, I\'m just gettin\' down \nSparkamatic, I\'m hangin\' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n\'89 in my time! You, \'90 is my year. \n\nYou\'re weakenin\' fast, YO! and I can tell it \nYour body\'s gettin\' hot, so, so I can smell it \nSo don\'t be mad and don\'t be sad \n\'Cause the lyrics belong to ICE, You can call me Dad \nYou\'re pitchin\' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don\'t be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you\'re dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n\u{4}\u{4}\u{4}\u{4}";
    let without_newlines: Vec<u8> = test_string
        .to_string()
        .bytes()
        .filter(|c| *c as char != '\n')
        .collect();
    let test_bytes: Vec<u8> = base64::decode(&without_newlines).unwrap();
    let res: Vec<u8> = cbc_dec(&test_bytes, &vec![0; 16], |c| {
        dec_aes_block(c, b"YELLOW SUBMARINE")
    });
    println!("{}", String::from_utf8(res.to_vec()).unwrap());
    assert_eq!(expected, String::from_utf8(res).unwrap());
}

/* challenge 11 */
fn n_randoms(n: u32) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let randoms: Vec<u8> = (0..n).map(|_| rng.gen()).collect();
    randoms
}

fn gen_random_aes_key() -> Vec<u8> {
    n_randoms(16)
}

#[test]
fn test_get_random_aes_key() {
    assert_eq!(gen_random_aes_key().len(), 16);
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum EncType {
    Ecb,
    Cbc,
}

fn cbc_or_ecb(plaintext: &[u8]) -> (Vec<u8>, EncType) {
    let mut rng = rand::thread_rng();
    let count = rng.gen_range(5..11);
    let mut pad_bytes: Vec<u8> = Vec::new();
    for _ in (0..count) {
        pad_bytes.push(rng.gen())
    }

    let padded_plaintext: Vec<u8> = [pad_bytes.clone(), plaintext.to_vec(), pad_bytes].concat();
    let key = gen_random_aes_key();

    let discrim = rng.gen_range(0..2);
    if discrim == 0 {
        // cbc
        let random_iv = n_randoms(16);
        (
            cbc_enc(plaintext, &random_iv, |c| enc_aes_block(c, &key)),
            EncType::Cbc,
        )
    } else {
        // ecb
        (enc_aes_block(plaintext, &key), EncType::Ecb)
    }
}

fn encryption_oracle(ciphertext: &[u8]) -> EncType {
    let mut seen: Vec<Vec<u8>> = Vec::new();
    for block in ciphertext.chunks(16) {
        if seen.contains(&block.to_vec()) {
            return EncType::Ecb; // any ciphertext containing 2 repeating blocks is automatically assumed to be ECB. Probably works most of the time? I'm not really sure what the probabilities are.
        }
        seen.push(block.to_vec());
    }
    return EncType::Cbc;
}

#[test]
fn test_enc_oracle() {
    let plaintext = "A".repeat(32);
    let (ciphertext, enctype) = cbc_or_ecb(plaintext.as_bytes());
    assert_eq!(encryption_oracle(&ciphertext), enctype)
}

fn repeats_from_start<T>(input: &[T]) -> usize
where
    T: Eq + Copy + std::fmt::Debug,
{
    let mut stack = Vec::new();
    let mut idx: usize = 1;
    stack.push(input[0]);
    while stack.len() > 0 && idx < input.len(){
        if stack[0] == input[idx] {
            let p = stack.pop().unwrap();
            println!("popped! {:#?}", p);
        } else {
            stack.push(input[idx]);
            println!("pushed! {:#?}", input[idx])
        }
        idx += 1;
    }
    idx
}

/* #[test]
fn test_repeats() {
    println!("{}", repeats_from_start("ACABACABACAB".as_bytes()));
    assert!(false)
} */

fn exercise12() {
    let secret = base64::decode(
        "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg
aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq
dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg
YnkK"
            .as_bytes(),
    )
    .unwrap();
    let plaintext = "A".repeat(32);
    let (ciphertext, enctype) = cbc_or_ecb(plaintext.as_bytes());
    assert_eq!(encryption_oracle(&ciphertext), enctype)
}
