// converts a character denoting a hex digit in 0123456789abcdef to
// the numerical value it represents
fn hex_digit(u: u8) -> Option<u8> {
    match u {
        48 ... 57 => Some(u - 48), // 0 - 9 represent 0 through 9
        65 ... 70 => Some(u - 55), // a - f represent 10 through 15
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

fn as_bytes(s: String) -> Option<Vec<u8>> {
    let chars = s.into_bytes();
    let pairs = chars.chunks(2);
    let nums: Vec<u8> = pairs.filter_map(|p| pair_to_num(p.to_vec())).collect();
    if nums.len() == chars.len() / 2 {
        Some(nums)
    } else {
        None
    }
}

fn byte_to_base64_char(u: u8) -> char {
    match u {
        41 ... 
    }
}

fn to_base64(Vec<u8>) -> String {
    
}

fn main() {
    println!("{:?}", as_bytes("hello, world!".to_string()));
}
