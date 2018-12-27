use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Point {
  pub x: i16,
  pub y: i16,
}

impl Point {
  pub fn distance_to(&self, other: &Point) -> u16 {
    (i16::abs(self.x - other.x) + i16::abs(self.y - other.y)) as u16
  }
}

impl FromStr for Point {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let coords: Vec<&str> = s.split(',').collect();

    let x_fromstr = coords[0].trim().parse::<i16>()?;
    let y_fromstr = coords[1].trim().parse::<i16>()?;

    Ok(Point {
      x: x_fromstr,
      y: y_fromstr,
    })
  }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Bounds {
  pub top: i16,
  pub bottom: i16,
  pub left: i16,
  pub right: i16,
}

impl Bounds {
  pub fn from_points(points: &[Point]) -> Bounds {
    let left: i16 = points.iter().fold(
      i16::max_value(),
      |min_x, p| if p.x < min_x { p.x } else { min_x },
    );

    let top: i16 = points.iter().fold(
      i16::max_value(),
      |min_y, p| if p.y < min_y { p.y } else { min_y },
    );

    let right: i16 = points.iter().fold(
      i16::min_value(),
      |max_x, p| if p.x > max_x { p.x } else { max_x },
    );

    let bottom: i16 = points.iter().fold(
      i16::min_value(),
      |max_y, p| if p.y > max_y { p.y } else { max_y },
    );

    Bounds {
      top,
      bottom,
      left,
      right,
    }
  }
}
