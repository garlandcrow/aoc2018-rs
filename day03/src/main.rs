use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

#[derive(Debug, PartialEq)]
struct Size {
  w: i32,
  h: i32,
}

#[derive(Debug, PartialEq)]
struct Claim {
  id: u16,
  loc: Point,
  size: Size,
}

impl Claim {
  fn from_str(line: &str) -> Result<Claim, Box<std::error::Error>> {
    // #1 @ 1,3: 4x4
    let split_chars = "#@,:x";
    let parts: Vec<&str> = line.split_terminator(|c| split_chars.contains(c)).collect();

    let id = u16::from_str(parts[1].trim()).unwrap();
    let loc = Point {
      x: i32::from_str(parts[2].trim()).unwrap(),
      y: i32::from_str(parts[3].trim()).unwrap(),
    };
    let size = Size {
      w: i32::from_str(parts[4].trim()).unwrap(),
      h: i32::from_str(parts[5].trim()).unwrap(),
    };
    Ok(Claim { id, loc, size })
  }
}

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
  let claims: Vec<Claim> = get_input("input.txt", |line| Claim::from_str(line).unwrap())?;
  println!("Part1: {}", part1(&claims));
  println!("Part2: {}", part2(&claims));
  Ok(())
}

fn part2(claims: &Vec<Claim>) -> u16 {
  let mut grid = [[0u16; 1001]; 1001];
  let mut untouched_claim_ids: HashSet<u16> = HashSet::with_capacity(1001);

  for c in claims {
    untouched_claim_ids.insert(c.id);

    for x in c.loc.x..(c.loc.x + c.size.w) {
      for y in c.loc.y..(c.loc.y + c.size.h) {
        let (i_x, i_y) = (x as usize, y as usize);
        let current_id = grid[i_x][i_y];
        if current_id != 0 {
          untouched_claim_ids.remove(&current_id);
          untouched_claim_ids.remove(&c.id);
        }
        grid[i_x][i_y] = c.id;
      }
    }
  }
  assert_eq!(untouched_claim_ids.len(), 1);

  *untouched_claim_ids.iter().next().unwrap()
}

fn part1(claims: &Vec<Claim>) -> i32 {
  let mut grid = [[0u8; 1001]; 1001];

  let mut overlaps = 0;

  for c in claims {
    for x in c.loc.x..(c.loc.x + c.size.w) {
      for y in c.loc.y..(c.loc.y + c.size.h) {
        let (i_x, i_y) = (x as usize, y as usize);
        if grid[i_x][i_y] == 1 {
          // only count first overlap
          overlaps += 1;
        }
        grid[i_x][i_y] += 1;
      }
    }
  }

  overlaps
}

#[test]
fn test_claim_from_str() {
  assert_eq!(
    Claim::from_str("#1 @ 1,3: 4x4").unwrap(),
    Claim {
      id: 1,
      loc: Point { x: 1, y: 3 },
      size: Size { w: 4, h: 4 },
    }
  );
}

#[test]
fn test_part_1() {
  let input: Vec<Claim> = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"
    .lines()
    .map(Claim::from_str)
    .map(Result::unwrap)
    .collect();
  assert_eq!(part1(&input), 4);
}

#[test]
fn test_part_2() {
  let input: Vec<Claim> = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"
    .lines()
    .map(Claim::from_str)
    .map(Result::unwrap)
    .collect();
  assert_eq!(part2(&input), 3);
}
