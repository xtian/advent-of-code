use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), &'static str> {
    let mut args = env::args().skip(1);

    let filename = args.next().ok_or("Input file required")?;
    let file = File::open(filename).map_err(|_| "Could not open input file")?;

    let output = find_overlap(file);

    println!("{}", output);

    Ok(())
}

fn find_overlap(input: impl Read) -> usize {
    BufReader::new(input)
        .lines()
        .filter_map(|line| line.ok())
        .fold(vec![vec![0; 1000]; 1000], |mut acc, line| {
            let (x, y, w, h) = parse_line(&line);

            for i in y..h + y {
                for j in x..w + x {
                    acc[i][j] += 1;
                }
            }

            acc
        }).iter()
        .flatten()
        .filter_map(|&n| if n > 1 { Some(1) } else { None })
        .sum()
}

fn parse_line(line: &str) -> (usize, usize, usize, usize) {
    let parts = line.split(' ').skip(2).collect::<Vec<_>>();

    let coords = parts[0]
        .trim_end_matches(':')
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let size = parts[1]
        .split('x')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();

    (coords[0], coords[1], size[0], size[1])
}

#[test]
fn finds_overlap() {
    let seq = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2".as_bytes();

    assert_eq!(find_overlap(seq), 4);
}
