use std::collections::HashMap;
use std::iter;
use super::single_xor::*;
use super::hex::*;

pub fn score(input: &[u8]) -> f64 {
    let mut char_freqs: HashMap<char, f64> =
        [('e',12.702),
         ('t',9.056),
         ('a',8.167),
         ('o',7.507),
         ('i',6.966),
         ('n',6.749),
         ('s',6.327),
         ('h',6.049),
         ('r',5.987),
         ('d',4.253),
         ('l',4.025),
         ('c',2.782),
         ('u',2.758),
         ('m',2.406),
         ('w',2.360),
         ('f',2.228),
         ('g',2.016),
         ('y',1.974),
         ('p',1.929),
         ('b',1.492),
         ('v',0.978),
         ('k',0.772),
         ('j',0.153),
         ('x',0.150),
         ('q',0.095),
         ('z',0.074),
         ('E',12.702),
         ('\u{0020}',4.000),
         ('\'',2.000),
         ('T',9.056),
         ('A',8.167),
         ('O',7.507),
         ('I',6.966),
         ('N',6.749),
         ('S',6.327),
         ('H',6.049),
         ('R',5.987),
         ('D',4.253),
         ('L',4.025),
         ('C',2.782),
         ('U',2.758),
         ('M',2.406),
         ('W',2.360),
         ('F',2.228),
         ('G',2.016),
         ('Y',1.974),
         ('P',1.929),
         ('B',1.492),
         ('V',0.978),
         ('K',0.772),
         ('J',0.153),
         ('X',0.150),
         ('Q',0.095),
         ('Z',0.074),].iter().cloned().collect();
    
    let mut sum: f64 = 0.0;
    for u in input {
        sum += *char_freqs.entry(*u as char).or_insert(-10.0);
    };
    sum
}

pub fn find_best(input: &[u8]) -> Option<Vec<u8>> {
    let with_scores = iter::repeat(input).zip(0 ..= std::u8::MAX).map(|(i, key)| { 
        let bytes = single_xor(i,key);
        let score = score(&bytes);
        (bytes, score)
    });
    let mut best = 0.0;
    let mut best_bytes: Option<Vec<u8>> = None;
    for (bytes, score) in with_scores {
        if score >= best {
            best = score;
            best_bytes = Some(bytes);
        }
    }
    best_bytes
}

pub fn find_best_key(input: &[u8]) -> Option<u8> {
    let with_scores = iter::repeat(input).zip(0 ..= std::u8::MAX).map(|(i, key)| { 
        let bytes = single_xor(i,key);
        (key, score(&bytes))
    });
    let mut best = 0.0;
    let mut best_key: Option<u8> = None;
    for (key, score) in with_scores {
        if score >= best {
            best = score;
            best_key = Some(key);
        }
    }
    best_key
}

#[test]
fn test_score() {
    assert_eq!(164.074, score("Cooking MC's like a pound of bacon".as_bytes()));
}

#[test]
fn test_find_best() {
    if let Some(hb) = hex_as_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736") {
        if let Some(b) = find_best(&hb) {
            if let Ok(s) = String::from_utf8(b) {
                assert_eq!("Cooking MC\'s like a pound of bacon".to_string(), s);
            } else {
                println!("unprintable result!");
                assert!(false);
            }
        } else {
            println!("find_best failed");
            assert!(false);
        }
    }
}
