use failure::{format_err, Error};
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

#[derive(Debug, Clone, Copy)]
struct Claim {
    id: i32,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
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
                    .and_then(|s| Ok(s.as_str().parse::<usize>()?))
            };

            Ok(Claim {
                id: cap_to_num(1)? as i32,
                x: cap_to_num(2)?,
                y: cap_to_num(3)?,
                width: cap_to_num(4)?,
                height: cap_to_num(5)?,
            })
        })
        .collect()
}

fn one() -> Result<(), Error> {
    let claims = read_input()?;
    let mut state = [[0i32; HEIGHT]; WIDTH];

    let collision = -1;

    let mut total = 0;
    for claim in claims {
        for i in claim.x..claim.x + claim.width {
            for j in claim.y..claim.y + claim.height {
                if state[i][j] == collision {
                    continue;
                } else if state[i][j] > 0 {
                    total += 1;
                    state[i][j] = collision;
                } else {
                    state[i][j] = claim.id;
                }
            }
        }
    }
    println!("3-1: {}", total);

    Ok(())
}

fn two() -> Result<(), Error> {
    let claims = read_input()?;
    let mut state = [[0i32; HEIGHT]; WIDTH];

    let collision = -1;

    let mut tainted_ids = HashSet::new();

    for claim in &claims {
        for i in claim.x..claim.x + claim.width {
            for j in claim.y..claim.y + claim.height {
                if state[i][j] == collision {
                    // Mark the current id as tainted.
                    tainted_ids.insert(claim.id);
                    continue;
                } else if state[i][j] > 0 {
                    tainted_ids.insert(state[i][j]);
                    tainted_ids.insert(claim.id);
                    state[i][j] = collision;
                } else {
                    state[i][j] = claim.id;
                }
            }
        }
    }

    let all_ids: HashSet<i32> = claims.iter().map(|claim| claim.id).collect();

    if let Some(answer) = all_ids.difference(&tainted_ids).next() {
        println!("3-2: {}", answer);
    } else {
        println!("3-2: nothing found!");
    }

    Ok(())
}

fn main() {
    one().unwrap();
    two().unwrap();
}
