
use failure::{format_err, Error};
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use chrono::prelude::*;

enum Action {
    FallsAsleep,
    WakesUp,
    BeginsShift(i32)
}

struct Observation {
    timestamp: DateTime<Local>, 
    action: Action
}

fn read_input() -> Result<Vec<Observation>, Error> {
    let input = File::open("input/day4-1.txt")?;
    let buffered = BufReader::new(input);

    let toplevel = Regex::new(r"\[(.*?)\]\s(.*)").unwrap();
    buffered.lines().map(|ln| {
        let line = ln?;

        let caps = re
            .captures(&line)
            .ok_or(format_err!("Failed parsing regex."))?;

    })
}

fn main() {

}