use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), &'static str> {
    let mut args = env::args().skip(1);

    let filename = args.next().ok_or("Input file required")?;
    let file = File::open(filename).map_err(|_| "Could not open input file")?;

    println!("{}", generate_checksum(file));

    Ok(())
}

fn generate_checksum(input: impl Read) -> usize {
    let (two_chars, three_chars) = BufReader::new(input)
        .lines()
        .filter_map(|line| line.ok().map(|l| l.chars().collect::<Vec<_>>()))
        .fold((0, 0), |(total_two_chars, total_three_chars), mut chars| {
            chars.sort();

            let mut map: HashMap<char, usize> = HashMap::new();

            for c in chars {
                *map.entry(c).or_insert(0) += 1;
            }

            let two_chars = if map.values().any(|&x| x == 2) { 1 } else { 0 };
            let three_chars = if map.values().any(|&x| x == 3) { 1 } else { 0 };

            (total_two_chars + two_chars, total_three_chars + three_chars)
        });

    two_chars * three_chars
}

#[test]
fn can_generate_checksum() {
    let seq = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab".as_bytes();

    assert_eq!(generate_checksum(seq), 12);
}
