use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), &'static str> {
    let mut args = env::args().skip(1);
    let filename = args.next().ok_or("Input file required")?;
    let file = File::open(filename).map_err(|_| "Could not open input file")?;

    let output = build_sequence(file);

    println!("{}", output);

    Ok(())
}

fn build_sequence(input: impl Read) -> String {
    let mut start = String::new();

    let graph = BufReader::new(input)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let words: Vec<_> = line.split(' ').collect();

            if start.is_empty() {
                start = words[1].to_string();
            }

            (words[1].to_string(), words[7].to_string())
        })
        .fold(HashMap::new(), |mut acc, (parent, child)| {
            let vec = acc.entry(parent).or_insert(Vec::new());
            vec.push(child);
            acc
        });

    traverse_graph(&start, &graph, HashSet::new())
}

fn traverse_graph(
    key: &String,
    graph: &HashMap<String, Vec<String>>,
    used: HashSet<String>,
) -> String {
    let mut children = graph.get(key).unwrap_or(&Vec::new()).clone();
    children.sort();

    let x: String = children
        .iter()
        .map(|key| traverse_graph(key, graph, used))
        .collect();

    format!("{}{}", key, x)
}

#[test]
fn builds_correct_sequence() {
    let seq = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
        .as_bytes();

    assert_eq!(build_sequence(seq), "CABDFE");
}
