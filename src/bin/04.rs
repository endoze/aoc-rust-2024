advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
  let matrix: Vec<Vec<&str>> = input
    .lines()
    .map(|line| line.char_indices().map(|(i, _)| &line[i..i + 1]).collect())
    .collect();

  let horizontal_count: u32 = input.lines().map(count_occurrences).sum();

  let vertical_count: u32 = rotate_90_degrees(&matrix)
    .iter()
    .map(|line| {
      let line = line.concat();
      count_occurrences(line)
    })
    .sum();

  let diag_right_count: u32 = rotate_45_degrees(&matrix)
    .iter()
    .map(|line| {
      let line = line.concat();
      count_occurrences(line)
    })
    .sum();

  let diag_left_count: u32 = rotate_135_degrees(&matrix)
    .iter()
    .map(|line| {
      let line = line.concat();
      count_occurrences(line)
    })
    .sum();

  Some(horizontal_count + vertical_count + diag_right_count + diag_left_count)
}

pub fn part_two(input: &str) -> Option<u32> {
  let mut a_indices: Vec<Coordinate> = vec![];

  let matrix: Vec<Vec<&str>> = input
    .lines()
    .enumerate()
    .map(|(y_coord, line)| {
      line
        .char_indices()
        .map(|(x_coord, char)| {
          if char == 'A' {
            a_indices.push(Coordinate::new(x_coord as i32, y_coord as i32));
          }

          &line[x_coord..x_coord + 1]
        })
        .collect()
    })
    .collect();

  let max_y: i32 = (matrix.len() as i32) - 1;
  let max_x: i32 = (matrix[0].len() as i32) - 1;

  let valid_xmas: u32 = a_indices
    .iter()
    .filter_map(|coord| {
      let corners = [
        coord.up_left_coord(),
        coord.up_right_coord(),
        coord.down_left_coord(),
        coord.down_right_coord(),
      ];

      if corners.iter().all(|c| c.valid(max_x, max_y)) {
        let chars: Vec<&str> = corners
          .iter()
          .map(|c| matrix[c.y as usize][c.x as usize])
          .collect();

        if matches!(
          &chars[..],
          ["M", "M", "S", "S"] | ["S", "M", "S", "M"] | ["S", "S", "M", "M"] | ["M", "S", "M", "S"]
        ) {
          Some(1)
        } else {
          None
        }
      } else {
        None
      }
    })
    .sum();

  Some(valid_xmas)
}

#[derive(Debug)]
struct Coordinate {
  pub x: i32,
  pub y: i32,
}

impl Coordinate {
  pub fn new(x: i32, y: i32) -> Self {
    Coordinate { x, y }
  }

  pub fn up_left_coord(&self) -> Self {
    Coordinate {
      x: self.x - 1,
      y: self.y - 1,
    }
  }

  pub fn up_right_coord(&self) -> Self {
    Coordinate {
      x: self.x + 1,
      y: self.y - 1,
    }
  }

  pub fn down_left_coord(&self) -> Self {
    Coordinate {
      x: self.x - 1,
      y: self.y + 1,
    }
  }

  pub fn down_right_coord(&self) -> Self {
    Coordinate {
      x: self.x + 1,
      y: self.y + 1,
    }
  }

  pub fn valid(&self, max_x: i32, max_y: i32) -> bool {
    self.x >= 0 && self.y >= 0 && self.x <= max_x && self.y <= max_y
  }
}

fn count_occurrences<T>(line: T) -> u32
where
  T: AsRef<str>,
{
  line.as_ref().matches("XMAS").count() as u32 + line.as_ref().matches("SAMX").count() as u32
}

fn rotate_45_degrees<'a>(matrix: &[Vec<&'a str>]) -> Vec<Vec<&'a str>> {
  let rows = matrix.len();
  let columns = matrix[0].len();

  (0..rows)
    .flat_map(|i| (0..columns).map(move |j| (i + j, matrix[i][j])))
    .fold(
      vec![Vec::new(); rows + columns - 1],
      |mut acc, (idx, value)| {
        acc[idx].push(value);
        acc
      },
    )
}

fn rotate_90_degrees<'a>(matrix: &[Vec<&'a str>]) -> Vec<Vec<&'a str>> {
  let rows = matrix.len();
  let columns = matrix[0].len();

  (0..columns)
    .map(|col| {
      (0..rows)
        .rev()
        .map(|row| matrix[row][col])
        .collect::<Vec<&str>>()
    })
    .collect()
}

fn rotate_135_degrees<'a>(matrix: &[Vec<&'a str>]) -> Vec<Vec<&'a str>> {
  rotate_45_degrees(&rotate_90_degrees(matrix))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(18));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(9));
  }
}
