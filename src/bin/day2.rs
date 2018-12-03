use failure::Error;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Result<Vec<String>, Error> {
    let input = File::open("input/day2-1.txt")?;
    let buffered = BufReader::new(input);

    buffered.lines().map(|ln| Ok(ln?)).collect()
}

type Occurences<'a> = &'a [(char, usize)];

fn occurrences(s: &str) -> Vec<(char, usize)> {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    chars
        .iter()
        .group_by(|c| **c)
        .into_iter()
        .map(|(key, grp)| (key, grp.count()))
        .collect()
}

fn has_n(s: Occurences, n: usize) -> bool {
    for (_c, count) in s {
        if *count == n {
            return true;
        }
    }
    false
}

fn has_two(s: Occurences) -> bool {
    has_n(s, 2)
}

fn has_three(s: Occurences) -> bool {
    has_n(s, 3)
}

fn one() -> Result<(), Error> {
    let (two, three) =
        read_input()?
            .iter()
            .map(|s| occurrences(&s))
            .fold((0, 0), |(mut two, mut three), val| {
                if has_two(&val) {
                    two += 1
                }

                if has_three(&val) {
                    three += 1
                }
                (two, three)
            });
    println!("2-1: {}", two * three);
    Ok(())
}

fn only_one_letter_off(a: &str, b: &str) -> Option<usize> {
    let (maybe_index, _) = a
        .chars()
        .zip(b.chars())
        // Track the maybe position of the char that is off.
        .fold_while((None, 0), |(seen, index), (a_val, b_val)| {
            if a_val == b_val {
                Continue((seen, index + 1))
            } else if seen == None {
                Continue((Some(index), index + 1))
            } else {
                // We saw more than 2 differences.
                Done((None, 0))
            }
        })
        .into_inner();
    maybe_index
}

fn two() -> Result<(), Error> {
    let input = read_input()?;
    for x in &input {
        for y in &input {
            if let Some(bad_index) = only_one_letter_off(&x, &y) {
                let answer: String = x
                    .chars()
                    .zip(0..)
                    .filter(|(_, index)| bad_index != *index)
                    .map(|e| e.0)
                    .collect();
                println!("2-2: {}", answer);
                return Ok(());
            }
        }
    }
    Ok(())
}

fn main() {
    one().unwrap();
    two().unwrap();
}
