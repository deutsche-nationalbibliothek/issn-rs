use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use issn::Issn;

fn main() {
    let file = File::open(env::args().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);
    let mut iter = reader.lines();
    let mut count = 0;

    while let Some(Ok(line)) = iter.next() {
        if Issn::new(line).is_err() {
            count += 1;
        }
    }

    println!("{count}");
}
