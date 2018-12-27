use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

mod types;
use crate::types::{Bounds, Point};

fn uniquely_shortest_point(from_point: &Point, to_points: &[Point]) -> Option<Point> {
  let mut shortest: (u16, Option<Point>) = (u16::max_value(), None);

  for p in to_points {
    let dist = p.distance_to(from_point);
    if dist == shortest.0 {
      shortest = (dist, None)
    } else if dist < shortest.0 {
      shortest = (dist, Some(p.clone()));
    }
  }

  shortest.1
}

fn find_largest_area(points: &[Point]) -> Option<usize> {
  let bounds = Bounds::from_points(points);
  let mut areas: HashMap<Point, Vec<Point>> = HashMap::with_capacity(points.len());

  for i in bounds.left..=bounds.right {
    for j in bounds.top..=bounds.bottom {
      let curr = Point { x: i, y: j };
      if let Some(p) = uniquely_shortest_point(&curr, &points) {
        let point_list = areas.entry(p).or_insert(vec![]);
        point_list.push(curr);
      }
    }
  }

  match areas
    .iter()
    .max_by(|(_k1, v1), (_k2, v2)| v1.len().cmp(&v2.len()))
  {
    Some((_k, v)) => Some(v.len()),
    _ => None,
  }
}

fn total_distance(from_point: &Point, to_points: &[Point]) -> u16 {
  let mut sum = 0;

  for p in to_points {
    sum += from_point.distance_to(p);
  }

  sum
}

fn safe_region_size(points: &[Point], threshold: u16) -> usize {
  let bounds = Bounds::from_points(points);
  let mut good_regions = vec![];

  for i in bounds.left..=bounds.right {
    for j in bounds.top..=bounds.bottom {
      let curr = Point { x: i, y: j };

      if total_distance(&curr, points) < threshold {
        good_regions.push(curr);
      }
    }
  }

  good_regions.len()
}

fn main() -> io::Result<()> {
  let f = File::open("input.txt")?;
  let f = BufReader::new(f);
  let points: Vec<Point> = f
    .lines()
    .map(|line| Point::from_str(&line.unwrap()).unwrap())
    .collect();

  if let Some(area_size) = find_largest_area(&points) {
    println!("Part1: Largest non-infinte area: {}", area_size);
  } else {
    println!("Part1: Failed to find area.");
  }

  println!(
    "Part2: Safe region size: {}",
    safe_region_size(&points, 10_000)
  );

  Ok(())
}

#[cfg(test)]
mod test_super {
  use super::*;

  #[test]
  fn test_distance_to() {
    assert_eq!(Point { x: 1, y: 1 }.distance_to(&Point { x: 3, y: 3 }), 4);
  }

  #[test]
  fn test_bounds() {
    let points = vec![
      Point { x: 1, y: 1 },
      Point { x: 1, y: 6 },
      Point { x: 8, y: 3 },
      Point { x: 3, y: 4 },
      Point { x: 5, y: 5 },
      Point { x: 8, y: 9 },
    ];
    assert_eq!(
      Bounds::from_points(&points),
      Bounds {
        top: 1,
        bottom: 9,
        left: 1,
        right: 8,
      }
    );
  }

  #[test]
  fn test_find_largest_area() {
    let points = vec![
      Point { x: 1, y: 1 },
      Point { x: 1, y: 6 },
      Point { x: 8, y: 3 },
      Point { x: 3, y: 4 },
      Point { x: 5, y: 5 },
      Point { x: 8, y: 9 },
    ];
    if let Some(len) = find_largest_area(&points) {
      assert_eq!(len, 17);
    }
  }

  #[test]
  fn test_safe_regions() {
    let points = vec![
      Point { x: 1, y: 1 },
      Point { x: 1, y: 6 },
      Point { x: 8, y: 3 },
      Point { x: 3, y: 4 },
      Point { x: 5, y: 5 },
      Point { x: 8, y: 9 },
    ];
    assert_eq!(safe_region_size(&points, 32), 16);
  }
}
