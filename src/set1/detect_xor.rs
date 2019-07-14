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

fn compare_scores(a: &[u8], b: &[u8]) -> Ordering {
    compare_f64(score(a), score(b))
}

fn find_xord_line(filename: &str) -> io::Result<String> {
    let f = File::open(filename)?;
    let f = BufReader::new(f);

    let line = f
        .lines()
        .sorted_by(|a, b| {
            compare_scores(
                &find_best(&a.as_ref().unwrap().as_bytes()).unwrap(),
                &find_best(&b.as_ref().unwrap().as_bytes()).unwrap(),
            )
        })
        .rev()
        .next()
        .unwrap()
        .unwrap();
    let out = format!("{}: {:#?}", line, find_best(&line.as_bytes()));
    Ok(out)
}

// #[test]
// TODO
// fn cryptopals_4() {
//     assert_eq!("7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f: nOW\u{0}THAT\u{0}THE\u{0}PARTY\u{0}IS\u{0}JUMPING*".to_string(), find_xord_line("/Users/j/cryptopals/src/set1/challenge4.txt").unwrap());
// }
