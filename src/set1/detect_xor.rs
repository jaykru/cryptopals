use std::cmp::Ordering;
use std::fs::File;
use std::io::BufRead;
use std::io::{self, BufReader};

use itertools::Itertools; // extents Iterator with all the itertools methods

use super::break_byte_xor::*;

fn compare_f64(a: f64, b: f64) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a == b {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

fn compare_scores(a: &Option<Vec<u8>>, b: &Option<Vec<u8>>) -> Ordering {
    let a_score = match a {
        None => 0.0,
        Some(ap) => score(ap),
    };
    let b_score = match b {
        None => 0.0,
        Some(bp) => score(bp),
    };
    compare_f64(a_score, b_score)
}

fn find_xord_line(filename: &str) -> io::Result<String> {
    let f = File::open(filename)?;
    let f = BufReader::new(f);

    let line = f
        .lines()
        .sorted_by(|b, a| {
            println!("a: {}", &a.as_ref().unwrap());
            println!("b: {}", &b.as_ref().unwrap());
            compare_scores(
                &find_best(a.as_ref().unwrap().as_bytes()),
                &find_best(b.as_ref().unwrap().as_bytes()),
            )
        })
        .next()
        .unwrap()
        .unwrap();
    let out = format!("{}: {:#?}", line, String::from_utf8(find_best(&line.as_bytes()).unwrap()).unwrap());
    Ok(out)
}

#[test]
fn cryptopals_4() {
     assert_eq!("7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f: nOW\u{0}THAT\u{0}THE\u{0}PARTY\u{0}IS\u{0}JUMPING*".to_string(), find_xord_line("/home/j/cryptopals/src/set1/4.txt").unwrap());
}
