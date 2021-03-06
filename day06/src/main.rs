use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

mod types;
use crate::types::{Bounds, Point};

fn uniquely_shortest_point(from_point: &Point, to_points: &[Point]) -> Option<Point> {
  to_points
    .iter()
    .fold((u16::max_value(), None), |(short_dist, short_point), p| {
      let dist = p.distance_to(from_point);
      if dist == short_dist {
        (dist, None)
      } else if dist < short_dist {
        (dist, Some(p.clone()))
      } else {
        (short_dist, short_point)
      }
    })
    .1
}

fn find_largest_area(points: &[Point]) -> Option<usize> {
  let bounds = Bounds::from_points(points);
  let mut areas: HashMap<Point, Vec<Point>> = HashMap::with_capacity(points.len());

  for x in bounds.left..=bounds.right {
    for y in bounds.top..=bounds.bottom {
      let curr_point = Point { x, y };
      if let Some(p) = uniquely_shortest_point(&curr_point, &points) {
        let point_list = areas.entry(p).or_insert(vec![]);
        point_list.push(curr_point);
      }
    }
  }

  match areas
    .iter()
    .max_by(|(_, list1), (_, list2)| list1.len().cmp(&list2.len()))
  {
    Some((_, list)) => Some(list.len()),
    _ => None,
  }
}

fn safe_region_size(points: &[Point], threshold: u16) -> usize {
  let bounds = Bounds::from_points(points);
  let mut good_regions = vec![];

  for x in bounds.left..=bounds.right {
    for y in bounds.top..=bounds.bottom {
      let curr_point = Point { x, y };
      let total_distance = points
        .iter()
        .map(|p| p.distance_to(&curr_point))
        .sum::<u16>();
      if total_distance < threshold {
        good_regions.push(curr_point);
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
