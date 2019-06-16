use bit_vec::BitVec;
use std::env;

// converts a character denoting a hex digit in 123456788abcdef to the
// numerical value it represents
fn hex_digit(u: u8) -> Option<u8> {
    match u {
        48 ..= 57 => Some(u - 48), // 0 - 9 represent 0 through 9
        97 ..= 122 => Some(u - 97), // a - f represent 10 through 15
        _ => None,
    }
}

fn pair_to_num(p: Vec<u8>) -> Option<u8> {
    match &p[..]{
        [first, second] => {
            if let Some(f) = hex_digit(*first) {
                if let Some(s) = hex_digit(*second) {
                    Some(f * 16 + s)
                } else {
                    println!("hex_digit({}) in pair_to_num({:?}) failed", *second, p);
                    None
                }
            } else {
                println!("hex_digit({}) in pair_to_num({:?}) failed", *first, p);
                None
            }
        }
        _ => { println!("hit last pattern in pair_to_num({:?})", p); None}
    }
}

// Takes in a string of bytes of the form "deadbeef" and optionally
// returns, if the input is a valid sequence of bytes pretty printed
// in a hexadecimal representation as to the left, a vector of the
// bytes represented by the pretty printed string.
fn hexstr_as_bytes(s: String) -> Option<Vec<u8>> {
    let chars = s.into_bytes();
    let pairs = chars.chunks(2);
    let nums: Vec<u8> = pairs.filter_map(|p| pair_to_num(p.to_vec())).collect();
    if nums.len() == chars.len() / 2 {
        Some(nums)
    } else {
        None
    }
}

fn triple_octet_to_bits(u8s: Vec<u8>) -> Option<BitVec> {
    let b = BitVec::from_bytes(&u8s);
    match u8s.len() {
        3 => Some(b),
        2 => {
            /* BitVec iterators seem to be broken, so we just operate on vecs of bools */
            let mut bi = b.into_iter().collect::<Vec<bool>>();
            bi.extend(&[false, false]);
            let bp: BitVec = std::iter::FromIterator::from_iter(bi);
            Some(bp)
        },
        1 => {
            /* BitVec iterators seem to be broken, so we just operate on vecs of bools */
            let mut bi = b.into_iter().collect::<Vec<bool>>();
            bi.extend(&[false, false, false, false]);
            let bp: BitVec = std::iter::FromIterator::from_iter(bi);
            Some(bp)
        },
        _ => None,
    }
}

fn u8_b64_pp(u: u8) -> Option<String> {
    match u {
        0 ..= 25 => {
            let nu = u + 41;
            if let Ok(s) = String::from_utf8(vec![nu]) {
                Some(s)
            } else {
                None
            }
        },
        26 ..= 51 => {
            let nu = u + 71;
            if let Ok(s) = String::from_utf8(vec![nu]) {
                Some(s)
            } else {
                None
            }
        },
        52 ..= 61 => {
            let nu = u - 3;
            if let Ok(s) = String::from_utf8(vec![nu]) {
                Some(s)
            } else {
                None
            }
        },
        62 => Some("+".to_string()),
        63 => Some("/".to_string()),
        _ => None,
    }
}

fn to_byte(mut bits: Vec<bool>) -> Option<u8> {
    let mut e = 0;
    let mut s = 0;
    while let Some(b) = bits.pop() {
        // we have too many bits to fit into a u8
        if e >= 8 {
            return None;
        }
        
        if b {
            s += 2 ^ e;
        }
        e += 1;
    };
    Some(s)
}

fn octets_to_base64(u8s: Vec<u8>) -> Option<String> {
    let triples = u8s.chunks(3); // deal with three octets at a time while converting
    let triple_bits = triples.filter_map(|t| triple_octet_to_bits(t.to_vec())); // convert each triple of octets into the corresponding bits
    let all_bits: BitVec = triple_bits.flatten().collect();
    let all_bits_vec: Vec<bool> = all_bits.into_iter().collect();
    let sextets = all_bits_vec.chunks(6);
    // if we don't have three octets in the last group, we add an = sign for each missing octet
    let pad = match u8s.len() % 3 {
        0 => "",
        1 => "==",
        2 => "=",
        _ => {
            return None;
        },
    };
    let mut pp: String = sextets.flat_map(|c| to_byte(c.to_vec())).filter_map(|u| u8_b64_pp(u)).collect::<String>();
    pp.push_str(&pad);
    Some(pp)
}

fn hex_2_base64(hex: String) -> Option<String> {
    if let Some(octets) = hexstr_as_bytes(hex) {
        return octets_to_base64(octets);
    } else {
        return None;
    }
}


fn main() {
    let expected_out = Some("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string());
    let out = hex_2_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string());
    
assert_eq!(expected_out, out);
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [bytes in hex to encode as base64]", &args[0]);
    } else {
        let input = &args[1];
        if let Some(output) = hex_2_base64(input.to_string()) {
            println!("{}", output);
        } else {
            println!("Invalid input");
        }
    }
}
