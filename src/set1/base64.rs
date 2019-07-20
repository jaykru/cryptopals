use super::hex::*;
///* set 1, challenge 1 *///
use bit_vec::BitVec;
use itertools::Itertools;

fn octets_to_b64bits(u8s: Vec<u8>) -> BitVec {
    let b = BitVec::from_bytes(&u8s);
    match u8s.len() % 3 {
        0 => b,
        1 => {
            /* BitVec iterators seem to be broken, so we just operate on vecs of bools */
            let mut bi = b.into_iter().collect::<Vec<bool>>();
            bi.extend(&[false, false, false, false]);
            let bp: BitVec = std::iter::FromIterator::from_iter(bi);
            bp
        }
        2 => {
            /* BitVec iterators seem to be broken, so we just operate on vecs of bools */
            let mut bi = b.into_iter().collect::<Vec<bool>>();
            bi.extend(&[false, false]);
            let bp: BitVec = std::iter::FromIterator::from_iter(bi);
            bp
        }
        _ => unreachable!(), // absurd
    }
}

fn u8_b64_pp(u: u8) -> Option<String> {
    match u {
        // 0 - 25 are capital letters.
        0..=25 => {
            let nu = u + 65;
            if let Ok(s) = String::from_utf8(vec![nu]) {
                Some(s)
            } else {
                unreachable!()
            }
        }
        26..=51 => {
            let nu = u + 71;
            if let Ok(s) = String::from_utf8(vec![nu]) {
                Some(s)
            } else {
                unreachable!();
            }
        }
        52..=61 => {
            let nu = u - 4;
            if let Ok(s) = String::from_utf8(vec![nu]) {
                Some(s)
            } else {
                unreachable!();
            }
        }
        62 => Some("+".to_string()),
        63 => Some("/".to_string()),
        _ => None,
    }
}

pub fn bits_to_byte(mut bits: Vec<bool>) -> Option<u8> {
    let mut e = 0;
    let mut s = 0;
    while let Some(b) = bits.pop() {
        // fail if we have too many bits to fit into a u8
        if e >= 8 {
            return None;
        }

        if b {
            s += 2_u8.pow(e);
        }
        e += 1;
    }
    Some(s)
}

fn octets_b64_pp(u8s: Vec<u8>) -> Option<String> {
    // if we don't have three octets in the last group, we add an = sign for each missing octet
    let pad = match u8s.len() % 3 {
        0 => "",
        1 => "==",
        2 => "=",
        _ => {
            // absurd
            return None;
        }
    };
    let sextets = octets_to_b64bits(u8s).into_iter().collect::<Vec<bool>>();
    let mut pp: String = sextets
        .chunks(6)
        .flat_map(|c| bits_to_byte(c.to_vec()))
        .filter_map(|u| u8_b64_pp(u))
        .collect::<String>();
    pp.push_str(&pad);
    Some(pp)
}

pub fn hex_2_base64(hex: &str) -> Option<String> {
    if let Some(octets) = hex_as_bytes(&hex) {
        return octets_b64_pp(octets);
    } else {
        return None;
    }
}

#[test]
fn challenge1() {
    let expected_out =
        Some("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string());
    let out = hex_2_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

    assert_eq!(expected_out, out);
}
