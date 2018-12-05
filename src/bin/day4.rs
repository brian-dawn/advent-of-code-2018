use chrono::prelude::*;
use failure::{format_err, Error};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type GuardId = i32;

#[derive(Debug, Clone, Copy)]
enum Observation {
    FallsAsleep(NaiveDateTime),
    WakesUp(NaiveDateTime),
    BeginsShift(NaiveDateTime, GuardId),
}

impl Observation {
    fn time(&self) -> NaiveDateTime {
        *match self {
            Observation::WakesUp(t) => t,
            Observation::FallsAsleep(t) => t,
            Observation::BeginsShift(t, _) => t
        }
    }
}

fn parse_line(line: &str) -> Result<Observation, Error> {
    let top_level = Regex::new(r"\[(.*)\]\s(.*)")?;

    let caps = top_level
        .captures(&line)
        .ok_or(format_err!("Failed parsing regex."))?;

    let timestamp = NaiveDateTime::parse_from_str(&caps[1], "%Y-%m-%d %H:%M")?;

    let action_str = &caps[2];

    let result = if action_str == "wakes up" {
        Observation::WakesUp(timestamp)
    } else if action_str == "falls asleep" {
        Observation::FallsAsleep(timestamp)
    } else {
        let id_find = Regex::new(r".*?(\d+).*")?;
        let caps = id_find.captures(action_str).ok_or(format_err!("Failed parsing action string."))?;
        let id = caps[1].to_owned().parse::<GuardId>()?;
        Observation::BeginsShift(timestamp, id)
    };
    Ok(result)
}

fn read_input() -> Result<Vec<Observation>, Error> {
    let input = File::open("input/day4-1.txt")?;
    let buffered = BufReader::new(input);

    buffered.lines().map(|ln| {
        parse_line(&ln?)
    }).collect()
}

fn one() -> Result<(), Error> {
    let mut input = read_input()?;
    input.sort_by(|a, b| {
        a.time().cmp(&b.time())
    });

    let (time_db, _, _) = input.iter()
        .fold((HashMap::new(), None, None), |(mut time_db, current_guard_id, latest_action), val| {
            let sleep_time = if let Some(Observation::FallsAsleep(prev_time)) = latest_action {
                // The prev action was sleep, count it up!
                val.time() - prev_time
            } else {
                // The prev action was awake so it's zero.
                time::Duration::zero()
            };

            // Update the sleep time in the time db.
            if let Some(id) = current_guard_id {
                let total_sleep_time = *time_db.get(id).unwrap_or(&time::Duration::zero());
                time_db.insert(id, total_sleep_time + sleep_time);

            }

            match val {
                Observation::WakesUp(_) => {
                    (time_db, current_guard_id, Some(*val))
                }
                Observation::FallsAsleep(_) => {
                    (time_db, current_guard_id, Some(*val))
                }
                Observation::BeginsShift(_, guard_id) => {
                    (time_db, Some(guard_id), Some(*val))
                }
            }
        });


    let longest_time = time_db.iter().fold(None, |acc, (id, duration)| {
        match acc {
            Some((longest_id, longest_time)) => {
                if longest_time < duration {
                    Some((id, duration))
                } else {
                    Some((longest_id, longest_time))
                }
            }
            None => Some((id, duration))

        }
    });

    if let Some((id, time)) = longest_time {
        let time = time.num_minutes() as i32;
        println!("4-1: {}", **id * time);
    } else {

        println!("4-1: Couldn't find anything.");
    }
    Ok(())
}

fn main() {
    one().unwrap()
}
