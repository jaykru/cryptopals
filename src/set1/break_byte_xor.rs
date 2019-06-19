use std::collections::HashMap;
use std::iter;
use super::single_xor::*;

fn score(input: &str) -> f64 {
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
         ('Z',0.074),
         ('\n',-100.0),
         ('*',-10.0),
         ('\'', 0.005),
         ('`', -1000.0)].iter().cloned().collect();
    
    let mut sum: f64 = 0.0;
    for u in input.chars() {
        sum += *char_freqs.entry(u).or_insert(-5.0);
    };
    sum
}

fn find_best(input: &str) -> String {
    let with_scores = iter::repeat(input).zip(1 ..= std::u8::MAX).map(|(i, key)| { 
        if let Ok(bytes) = single_xor(i,key) {
            if let Ok(dec) = std::str::from_utf8(&bytes) {
                println!("{} [score {}] [key {:x}]", dec, score(dec), key);
                (String::from(dec), score(dec))
            } else {
                (String::from(""), -1.0)
            }
        } else {
            unreachable!();
        }});
    let mut best = 0.0;
    let mut best_dec = String::from("");
    for (dec, score) in with_scores {
        if score >= best {
            best = score;
            best_dec = dec;
        }
    }

    best_dec
}

#[test]
fn test_score() {
    assert_eq!(108.079, score("Cooking MC's like a pound of bacon"));
}

#[test]
fn test_find_best() {
    assert_eq!("Cooking MC\'s like a pound of bacon".to_string(), find_best("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"));
}
