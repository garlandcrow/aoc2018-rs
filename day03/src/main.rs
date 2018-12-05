use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

fn get_input<T, F>(filename: &str, transform: F) -> io::Result<Vec<T>>
where
  F: Fn(&str) -> T,
{
  let f = File::open(filename)?;
  let f = BufReader::new(f);
  let input: Vec<T> = f.lines().map(|line| transform(&line.unwrap())).collect();
  Ok(input)
}

fn main() -> io::Result<()> {
  let input: Vec<i32> = get_input("input.txt", |l| i32::from_str(l).unwrap())?;
  println!("{:?}", input);
  Ok(())
}
