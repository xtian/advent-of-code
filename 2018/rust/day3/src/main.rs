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
        "overlap" => Ok(find_overlap(file)),
        "non-overlapped" => find_non_overlapped(file).ok_or(""),
        _ => Err("Unrecognized command"),
    }?;

    println!("{}", output);

    Ok(())
}

fn find_overlap(input: impl Read) -> usize {
    get_inputs(input)
        .fold(vec![vec![0; 1000]; 1000], |mut acc, line| {
            let p = parse_line(&line);

            for i in p.y..p.h + p.y {
                for j in p.x..p.w + p.x {
                    acc[i][j] += 1;
                }
            }

            acc
        })
        .iter()
        .flatten()
        .filter_map(|n| if *n > 1 { Some(1) } else { None })
        .sum()
}

fn find_non_overlapped(input: impl Read) -> Option<usize> {
    let lines: Vec<_> = get_inputs(input).map(|line| parse_line(&line)).collect();

    let grid = lines.iter().fold(vec![vec![0; 1000]; 1000], |mut acc, p| {
        for i in p.y..p.h + p.y {
            for j in p.x..p.w + p.x {
                acc[i][j] += 1;
            }
        }

        acc
    });

    lines
        .iter()
        .find(|p| {
            let mut found = true;

            for i in p.y..p.h + p.y {
                for j in p.x..p.w + p.x {
                    if grid[i][j] != 1 {
                        found = false;
                    }
                }
            }

            found
        })
        .map(|piece| piece.id)
}

struct Piece {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

fn get_inputs(input: impl Read) -> impl Iterator<Item = String> {
    BufReader::new(input).lines().filter_map(|line| line.ok())
}

fn parse_line(line: &str) -> Piece {
    let parts = line.split(' ').collect::<Vec<_>>();

    let id: usize = parts[0].replace("#", "").parse().unwrap();

    let coords = parts[2]
        .trim_end_matches(':')
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let size = parts[3]
        .split('x')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();

    Piece {
        id,
        x: coords[0],
        y: coords[1],
        w: size[0],
        h: size[1],
    }
}

#[test]
fn finds_overlap() {
    let seq = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2".as_bytes();

    assert_eq!(find_overlap(seq), 4);
}

#[test]
fn finds_non_overlapped() {
    let seq = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2".as_bytes();

    assert_eq!(find_non_overlapped(seq), Some(3));
}
