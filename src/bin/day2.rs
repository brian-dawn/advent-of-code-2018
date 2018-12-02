
use failure::Error;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Result<Vec<String>, Error> {
    let input = File::open("input/day2-1.txt")?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|ln| Ok(ln?))
        .collect()
}

type Occurences = Vec<(char, usize)>;
fn occurrences(s: &str) -> Occurences{
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    chars.iter().group_by(|c| **c)
    .into_iter()
    .map(|(key, grp)| (key, grp.count()))
    .collect()
}

fn has_n(s: &Occurences, n:usize) -> bool {
    for (_c, count) in s{
        if *count == n {
            return true;
        }
    }
    false
}

fn has_two(s: &Occurences) -> bool {
    has_n(s, 2)
}

fn has_three(s: &Occurences) -> bool {
    has_n(s, 3)
}

fn one() -> Result<(), Error> {
    let (two, three) = read_input()?
    .iter()
    .map(|s| occurrences(&s))
    .fold((0, 0), |(mut two, mut three), val| {

        if has_two(&val) {
            two = two + 1;
        }

        if has_three(&val) {
            three = three + 1;
        }
        (two, three)
    });
    println!("2-1: {}", two * three);
    Ok(())
}

fn main() {
    one().unwrap();
}