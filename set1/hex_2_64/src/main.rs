use bit_vec::BitVec;
use std::env;

// converts a character denoting a hex digit in 123456788abcdef to the
// numerical value it represents

fn hex_digit(u: u8) -> Option<u8> {
    match u {
        48 ..= 57 => Some(u - 48), // 0 - 9 represent 0 through 9
        97 ..= 122 => Some(u - 87), // a - f represent 10 through 15
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
                    None
                }
            } else {
                None
            }
        }
        _ => None
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
        for i in &nums {
            format!("{:x}", i);
        }
        Some(nums)
    } else {
        unreachable!();
    }
}

#[test]
fn test_hex_str_bytes() {
    assert_eq!(hexstr_as_bytes("49276d206b696c6c".to_string()), Some(vec![0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c, 0x6c]));
}

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
        },
        2 => {
            /* BitVec iterators seem to be broken, so we just operate on vecs of bools */
            let mut bi = b.into_iter().collect::<Vec<bool>>();
            bi.extend(&[false, false]);
            let bp: BitVec = std::iter::FromIterator::from_iter(bi);
            bp
        },
        _ => unreachable!(), // absurd
    }
}

fn u8_b64_pp(u: u8) -> Option<String> {
    match u {
        // 0 - 25 are capital letters.
        0 ..= 25 => {
            let nu = u + 65;
            if let Ok(s) = String::from_utf8(vec![nu]) {
                Some(s)
            } else {
                unreachable!()
            }
        },
        26 ..= 51 => {
            let nu = u + 71;
            if let Ok(s) = String::from_utf8(vec![nu]) {
                Some(s)
            } else {
                unreachable!();
            }
        },
        52 ..= 61 => {
            let nu = u - 4;
            if let Ok(s) = String::from_utf8(vec![nu]) {
                Some(s)
            } else {
                unreachable!();
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
        // fail if we have too many bits to fit into a u8
        if e >= 8 {
            return None;
        }
        
        if b {
            s += 2_u8.pow(e);
        }
        e += 1;
    };
    Some(s)
}

fn octets_b64_pp(u8s: Vec<u8>) -> Option<String> {
    // if we don't have three octets in the last group, we add an = sign for each missing octet
    let pad = match u8s.len() % 3 {
        0 => "",
        1 => "==",
        2 => "=",
        _ => { // absurd
            return None;
        },
    };
    let sextets = octets_to_b64bits(u8s).into_iter().collect::<Vec<bool>>();
    let mut pp: String = sextets.chunks(6).flat_map(|c| to_byte(c.to_vec())).filter_map(|u| u8_b64_pp(u)).collect::<String>();
    pp.push_str(&pad);
    Some(pp)
}

fn hex_2_base64(hex: String) -> Option<String> {
    if let Some(octets) = hexstr_as_bytes(hex) {
        return octets_b64_pp(octets);
    } else {
        return None;
    }
}

#[test]
fn test2() {
    if let Some(ex) = hex_2_base64("41".to_string()) {
        assert_eq!("QQ==", ex);
    }

}

#[test]
fn main_test() {
    let expected_out = Some("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string());
    let out = hex_2_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string());
    
    assert_eq!(expected_out, out);
}

fn main() {
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
