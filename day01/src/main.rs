use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

fn load_input<T, F>(filename: &str, transform: F) -> io::Result<Vec<T>>
where
  F: Fn(&str) -> T,
{
  let f = File::open(filename)?;
  let f = BufReader::new(f);
  let input: Vec<T> = f.lines().map(|line| transform(&line.unwrap())).collect();
  Ok(input)
}

fn get_input() -> io::Result<Vec<i32>> {
  load_input("input.txt", |line| i32::from_str(line).unwrap())
}

fn main() -> io::Result<()> {
  let input = get_input()?;
  println!("Part1: {}", part1(&input)?);
  println!("Part2: {}", part2(&input)?);
  Ok(())
}

fn part1(input: &Vec<i32>) -> io::Result<i32> {
  let mut sum = 0;
  for freq in input {
    sum += freq;
  }
  Ok(sum)
}

fn part2(input: &Vec<i32>) -> io::Result<i32> {
  let mut seen_freq = HashSet::new();
  seen_freq.insert(0);

  let mut sum = 0;

  loop {
    for freq in input {
      sum += freq;
      if seen_freq.contains(&sum) {
        return Ok(sum);
      } else {
        seen_freq.insert(sum);
      }
    }
  }
}
