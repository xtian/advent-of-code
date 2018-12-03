use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), &'static str> {
    let mut args = env::args().skip(1);

    let command = args.next().ok_or("Command required")?;
    let filename = args.next().ok_or("Input file required")?;
    let file = File::open(filename).map_err(|_| "Could not open input file")?;

    let output = match command.as_str() {
        "sum" => Ok(sum_lines(file)),
        "repeated" => Ok(find_repeated_value(file)),
        _ => Err("Unrecognized command"),
    }?;

    println!("{}", output);

    Ok(())
}

fn sum_lines(input: impl Read) -> isize {
    get_inputs(input).sum()
}

fn find_repeated_value(input: impl Read) -> isize {
    let mut current_value = 0;
    let mut past_values = HashSet::new();

    past_values.insert(0);

    let inputs = get_inputs(input).collect::<Vec<_>>();
    let mut cycle = inputs.iter().cycle();

    loop {
        current_value += cycle.next().unwrap();

        if past_values.contains(&current_value) {
            return current_value;
        } else {
            past_values.insert(current_value);
        }
    }
}

fn get_inputs(input: impl Read) -> impl Iterator<Item = isize> {
    BufReader::new(input)
        .lines()
        .filter_map(|line| line.ok().and_then(|l| l.parse::<isize>().ok()))
}

#[test]
fn finds_repeated_values() {
    let seq = "+1\n-1".as_bytes();
    assert_eq!(find_repeated_value(seq), 0);

    let seq = "+3\n+3\n+4\n-2\n-4".as_bytes();
    assert_eq!(find_repeated_value(seq), 10);

    let seq = "-6\n+3\n+8\n+5\n-6".as_bytes();
    assert_eq!(find_repeated_value(seq), 5);

    let seq = "+7\n+7\n-2\n-7\n-4".as_bytes();
    assert_eq!(find_repeated_value(seq), 14);
}

#[test]
fn sums_lines() {
    let seq = "0".as_bytes();
    assert_eq!(sum_lines(seq), 0);

    let seq = "+4\n-2\n-1\n".as_bytes();
    assert_eq!(sum_lines(seq), 1);
}
