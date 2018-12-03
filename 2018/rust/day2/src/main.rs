use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), &'static str> {
    let mut args = env::args().skip(1);

    let command = args.next().ok_or("Command required")?;
    let filename = args.next().ok_or("Input file required")?;
    let file = File::open(filename).map_err(|_| "Could not open input file")?;

    match command.as_str() {
        "checksum" => {
            println!("{}", generate_checksum(file));
            Ok(())
        }
        "matches" => {
            let output = find_matching_string(file).ok_or("No match found")?;
            println!("{}", output);
            Ok(())
        }
        _ => Err("Unrecognized command"),
    }
}

fn find_matching_string(input: impl Read) -> Option<String> {
    let input: Vec<_> = BufReader::new(input)
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    for line_a in input.iter() {
        for line_b in input.iter() {
            let diff = line_a.char_indices().zip(line_b.chars()).fold(
                vec![],
                |mut acc, ((index, char_a), char_b)| {
                    if char_a != char_b {
                        acc.push(index);
                    }

                    acc
                },
            );

            if diff.len() == 1 {
                let (left, right) = line_a.split_at(diff[0]);
                let right: String = right.chars().skip(1).collect();

                return Some(format!("{}{}", left, right));
            }
        }
    }

    None
}

fn generate_checksum(input: impl Read) -> usize {
    let (two_chars, three_chars) = BufReader::new(input)
        .lines()
        .filter_map(|line| line.ok())
        .fold((0, 0), |(total_two_chars, total_three_chars), line| {
            let mut map: HashMap<char, usize> = HashMap::new();

            for c in line.chars() {
                *map.entry(c).or_insert(0) += 1;
            }

            let two_chars = if map.values().any(|&x| x == 2) { 1 } else { 0 };
            let three_chars = if map.values().any(|&x| x == 3) { 1 } else { 0 };

            (total_two_chars + two_chars, total_three_chars + three_chars)
        });

    two_chars * three_chars
}

#[test]
fn can_find_matching_strings() {
    let seq = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz".as_bytes();

    assert_eq!(find_matching_string(seq), Some("fgij".to_string()));
}

#[test]
fn can_generate_checksum() {
    let seq = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab".as_bytes();

    assert_eq!(generate_checksum(seq), 12);
}
