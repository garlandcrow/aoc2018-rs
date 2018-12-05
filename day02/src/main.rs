use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_input(filename: &str) -> io::Result<Vec<String>> {
  let f = File::open(filename)?;
  let f = BufReader::new(f);
  let input: Vec<String> = f.lines().map(|line| line.unwrap()).collect();
  Ok(input)
}

fn main() -> io::Result<()> {
  let input = get_input("input.txt")?;
  println!("Part1: {}", part1(&input));
  // println!("Part2: {}", part2(&input));
  Ok(())
}

fn has_pairs_trips(id: &str) -> (bool, bool) {
  let mut char_counts = HashMap::new();

  id.chars().for_each(|c| {
    let count = char_counts.entry(c).or_insert(0);
    *count += 1;
  });

  char_counts.iter().fold(
    (false, false),
    |(has_pairs, has_trips), (_c, count)| match *count {
      2 => (true, has_trips),
      3 => (has_pairs, true),
      _ => (has_pairs, has_trips),
    },
  )
}

fn part1(input: &Vec<String>) -> i32 {
  let (pair_count, trip_count) = input.iter().fold((0, 0), |(p_count, t_count), boxid| {
    match has_pairs_trips(&boxid) {
      (true, true) => (p_count + 1, t_count + 1),
      (true, false) => (p_count + 1, t_count),
      (false, true) => (p_count, t_count + 1),
      _ => (p_count, t_count),
    }
  });
  pair_count * trip_count
}

fn part2(input: &Vec<String>) -> &str {
  ""
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part1() {
    assert_eq!(has_pairs_trips("abcdef"), (false, false));
    assert_eq!(has_pairs_trips("bababc"), (true, true));
    assert_eq!(has_pairs_trips("abbcde"), (true, false));
    assert_eq!(has_pairs_trips("abcccd"), (false, true));
    assert_eq!(has_pairs_trips("aabcdd"), (true, false));
    assert_eq!(has_pairs_trips("abcdee"), (true, false));
    assert_eq!(has_pairs_trips("ababab"), (false, true));
  }
  #[test]
  fn test_part2() {
    let input = vec![
      "abcde".to_string(),
      "fghij".to_string(),
      "klmno".to_string(),
      "pqrst".to_string(),
      "fguij".to_string(),
      "axcye".to_string(),
      "wvxyz".to_string(),
    ];
    assert_eq!(part2(&input), "fgij");
  }
}
