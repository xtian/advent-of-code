use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), &'static str> {
    let filename = env::args().skip(1).next().ok_or("Input file required")?;
    let file = File::open(filename).map_err(|_| "Could not open input file")?;

    println!("{}", sum_lines(file));

    Ok(())
}

fn sum_lines(input: impl Read) -> isize {
    BufReader::new(input)
        .lines()
        .filter_map(|line| line.ok())
        .fold(0, |acc, line| acc + line.parse::<isize>().unwrap_or(0))
}

#[test]
fn handles_zero() {
    let seq = "0".as_bytes();

    assert_eq!(sum_lines(seq), 0);
}

#[test]
fn handles_sequence() {
    let seq = "+4\n-2\n-1\n".as_bytes();

    assert_eq!(sum_lines(seq), 1);
}
