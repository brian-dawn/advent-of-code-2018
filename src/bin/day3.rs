use failure::{format_err, Error};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

#[derive(Debug, Clone, Copy)]
struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn read_input() -> Result<Vec<Claim>, Error> {
    let input = File::open("input/day3-1.txt")?;
    let buffered = BufReader::new(input);

    let re = Regex::new(r"#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
    buffered
        .lines()
        .map(|ln| {
            let line = ln?;
            let caps = re
                .captures(&line)
                .ok_or(format_err!("Failed parsing regex."))?;

            let cap_to_num = |index| {
                caps.get(index)
                    .ok_or(format_err!("Failed to parse group."))
                    .and_then(|s| Ok(s.as_str().parse::<i32>()?))
            };

            Ok(Claim {
                id: cap_to_num(1)?,
                x: cap_to_num(2)?,
                y: cap_to_num(3)?,
                width: cap_to_num(4)?,
                height: cap_to_num(5)?,
            })
        })
        .collect()
}

fn main() {
    println!("{:?}", read_input().unwrap()[0]);
}
