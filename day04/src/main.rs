extern crate chrono;

use chrono::prelude::*;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum ParseEventError {
  ParseIntError(ParseIntError),
  BadInput,
}

impl From<ParseIntError> for ParseEventError {
  fn from(err: ParseIntError) -> Self {
    ParseEventError::ParseIntError(err)
  }
}

// type id = i32;

// #[derive(Debug)]
// enum Event {
//   ClockIn(id, DateTime<Utc>),
//   FallAsleep(DateTime<Utc>),
//   WakeUp(DateTime<Utc>),
// }

#[derive(Debug)]
struct Event {
  x: i32,
  y: i32,
}

impl FromStr for Event {
  type Err = ParseEventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use self::ParseEventError::*;

    let mut coords = s.splitn(2, ",");
    let x = coords.next().ok_or(BadInput)?.trim().parse()?;
    let y = coords.next().ok_or(BadInput)?.trim().parse()?;
    debug_assert!(coords.next().is_none());

    Ok(Event { x, y })
  }
}

fn get_input<T>(filename: &str) -> io::Result<Vec<T>>
where
  T: FromStr,
  <T as FromStr>::Err: Debug,
{
  let f = File::open(filename)?;
  let f = BufReader::new(f);
  let input: Vec<T> = f
    .lines()
    .map(|line| line.unwrap().parse::<T>().unwrap())
    .collect();
  Ok(input)
}

fn main() -> io::Result<()> {
  let input: Vec<Event> = get_input("input.txt")?;
  println!("{:?}", &input);
  Ok(())
}
