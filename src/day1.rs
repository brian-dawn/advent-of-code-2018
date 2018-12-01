use failure::Error;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Result<Vec<i32>, Error> {
    let input = File::open("input/day1-1.txt")?;
    let buffered = BufReader::new(input);

    // Collect will take our iter of results and make it
    // a result of the inner thing, neat.
    buffered
        .lines()
        .map(|line| Ok(line?.parse::<i32>()?))
        .collect()
}

fn one() -> Result<(), Error> {
    let answer: i32 = read_input()?.iter().sum();
    println!("1-1: {}", answer);
    Ok(())
}

fn two() -> Result<(), Error> {
    let input = read_input();
    let (answer, _) = input?
        .iter()
        .cycle()
        .fold_while((0, HashSet::new()), |(current_freq, mut observed), val| {
            if observed.contains(&current_freq) {
                Done((current_freq, observed))
            } else {
                observed.insert(current_freq);
                Continue((current_freq + val, observed))
            }
        }).into_inner();
    println!("1-2: {}", answer);
    Ok(())
}

pub fn run() {
    one().unwrap();
    two().unwrap();
}
