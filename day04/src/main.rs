#![feature(try_trait)]

extern crate regex;
extern crate time;

use regex::Regex;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;
use time::Tm;

#[derive(Debug)]
enum ParseEventError {
  ParseIntError(ParseIntError),
  ParseIdError(regex::Error),
  NoMatchError(std::option::NoneError),
  ParseTimeError(time::ParseError),
  BadInput,
}

impl From<ParseIntError> for ParseEventError {
  fn from(err: ParseIntError) -> Self {
    ParseEventError::ParseIntError(err)
  }
}

impl From<time::ParseError> for ParseEventError {
  fn from(err: time::ParseError) -> Self {
    ParseEventError::ParseTimeError(err)
  }
}

impl From<regex::Error> for ParseEventError {
  fn from(err: regex::Error) -> Self {
    ParseEventError::ParseIdError(err)
  }
}

impl From<std::option::NoneError> for ParseEventError {
  fn from(err: std::option::NoneError) -> Self {
    ParseEventError::NoMatchError(err)
  }
}

#[derive(Debug)]
enum Event {
  ClockIn(i32, Tm),
  FallAsleep(Tm),
  WakeUp(Tm),
}

impl FromStr for Event {
  type Err = ParseEventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use self::ParseEventError::*;

    // [1518-11-01 00:00] Guard #10 begins shift
    let mut split_itr = s.splitn(2, "] ");

    let dt_str = split_itr.next().ok_or(BadInput)?;
    let dt = time::strptime(dt_str, "[%Y-%m-%d %H:%M")?;

    let mut meta_itr = split_itr.next().ok_or(BadInput)?.splitn(2, " ");

    match meta_itr.next().ok_or(BadInput)? {
      "falls" => Ok(Event::FallAsleep(dt)),
      "wakes" => Ok(Event::WakeUp(dt)),
      "Guard" => {
        let guard_info = meta_itr.next().ok_or(BadInput)?;
        let id_re = Regex::new(r"^.*?#(?P<id>\d+?)\s.*?$")?;
        let caps = id_re.captures(guard_info)?;
        let id = caps.name("id")?.as_str().parse::<i32>()?;
        Ok(Event::ClockIn(id, dt))
      }
      _ => Err(BadInput),
    }
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
  for x in input {
    println!("{:?}", x);
  }
  Ok(())
}
