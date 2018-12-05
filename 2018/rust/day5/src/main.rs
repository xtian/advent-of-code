use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), &'static str> {
    let mut args = env::args().skip(1);

    let filename = args.next().ok_or("Input file required")?;
    let file = File::open(filename).map_err(|_| "Could not open input file")?;

    let mut string = String::new();

    BufReader::new(file)
        .read_line(&mut string)
        .map_err(|_| "Could not read from file")?;

    println!("{}", process_polymer(string).trim().len());

    Ok(())
}

fn process_polymer(mut polymer: String) -> String {
    let pairs: Vec<_> = (b'a'..=b'z')
        .map(|c| c as char)
        .flat_map(|c| {
            vec![
                format!("{}{}", c, c.to_uppercase()),
                format!("{}{}", c.to_uppercase(), c),
            ]
        }).collect();

    while pairs.iter().any(|s| polymer.contains(s)) {
        for s in pairs.iter() {
            polymer = polymer.replace(s, "");
        }
    }

    polymer
}

#[test]
fn processes_polymer() {
    assert_eq!(process_polymer("aA".to_string()), "");
    assert_eq!(process_polymer("abBA".to_string()), "");
    assert_eq!(process_polymer("aabAAB".to_string()), "aabAAB");
    assert_eq!(
        process_polymer("dabAcCaCBAcCcaDA".to_string()),
        "dabCBAcaDA"
    );
}
