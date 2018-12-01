use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashSet;

fn one() {
    let input = File::open("../input/day1-1.txt").unwrap();
    let buffered = BufReader::new(input);

    let answer = buffered
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .fold(0, |acc, val| acc + val);

    println!("1-1: {}", answer);
}

fn two() {
    let input = File::open("../input/day1-1.txt").unwrap();
    let buffered = BufReader::new(input);

    let (answer, _) = buffered
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .iter()
        .cycle()
        .fold_while(
            (0, HashSet::new()),
            |(current_freq, mut observed): (i32, HashSet<i32>), val| {
                if observed.contains(&current_freq) {
                    Done((current_freq, observed))
                } else {
                    observed.insert(current_freq);
                    Continue((current_freq + val, observed))
                }
            },
        ).into_inner();
    println!("1-2: {}", answer);
}

pub fn run() {
    one();
    two();
}
}
