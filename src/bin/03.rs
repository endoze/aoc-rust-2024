advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
  let regex = Regex::new(r"(?x)mul\((?<op1>[0-9]{1,3}),(?<op2>[0-9]{1,3})\)").unwrap();

  let results: u32 = regex
    .captures_iter(input)
    .filter_map(|cap| {
      let multiplicand = cap["op1"].parse::<u32>().ok()?;
      let multiplier = cap["op2"].parse::<u32>().ok()?;

      Some(multiplicand * multiplier)
    })
    .sum();

  Some(results)
}

pub fn part_two(input: &str) -> Option<u32> {
  let regex =
    Regex::new(r"(?x) (?<dont>don't) | (?<do>do) | mul\((?<op1>[0-9]{1,3}),(?<op2>[0-9]{1,3})\)")
      .unwrap();

  let mut include_instruction = true;

  let results: u32 = regex
    .captures_iter(input)
    .filter_map(|cap| {
      let reg_match = &cap[0];

      match (reg_match, include_instruction) {
        ("don't", _) => {
          include_instruction = false;
          None
        }
        ("do", _) => {
          include_instruction = true;
          None
        }
        (_, true) => {
          let multiplicand = cap["op1"].parse::<u32>().ok()?;
          let multiplier = cap["op2"].parse::<u32>().ok()?;

          Some(multiplicand * multiplier)
        }
        _ => None,
      }
    })
    .sum();

  Some(results)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(161));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(48));
  }
}
