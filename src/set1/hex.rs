// converts a character denoting a hex digit in 123456788abcdef to the
// numerical value it represents
fn hex_digit(u: u8) -> Option<u8> {
    match u {
        48 ..= 57 => Some(u - 48), // 0 - 9 represent 0 through 9
        97 ..= 102 => Some(u - 87), // a - f represent 10 through 15
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

pub fn hex_as_bytes(s: &str) -> Option<Vec<u8>> {
    let chars = s.as_bytes();
    let pairs = chars.chunks(2);
    let nums: Vec<u8> = pairs.filter_map(|p| pair_to_num(p.to_vec())).collect();
    if nums.len() == chars.len() / 2 {
        for i in &nums {
            format!("{:x}", i);
        }
        Some(nums)
    } else {
        None
    }
}

#[test]
fn test_hex_as_bytes() {
    assert_eq!(hex_as_bytes("49276d206b696c6c"), Some(vec![0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c, 0x6c]));
}
