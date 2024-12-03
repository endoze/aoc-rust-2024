advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
  let (mut left_side, mut right_side) = parse_string_into_lists(input);

  Some(calculate_distance(&mut left_side, &mut right_side))
}

pub fn part_two(input: &str) -> Option<u32> {
  let (left_side, right_side) = parse_string_into_lists(input);

  Some(calculate_similarity(&left_side, &right_side))
}

fn calculate_distance(left: &mut [u32], right: &mut [u32]) -> u32 {
  let mut distances: Vec<u32> = vec![];

  left.sort();
  right.sort();

  left.iter().zip(right.iter()).for_each(|(l, r)| {
    distances.push(l.abs_diff(*r));
  });

  distances.iter().sum()
}

fn calculate_similarity(left: &[u32], right: &[u32]) -> u32 {
  let mut occurences = HashMap::<u32, u32>::new();
  let mut similarity: Vec<u32> = vec![];

  right.iter().for_each(|num| {
    occurences.entry(*num).and_modify(|e| *e += 1).or_insert(1);
  });

  left.iter().for_each(|num| {
    let count_for_num = occurences.get(num);

    if let Some(count_for_num) = count_for_num {
      similarity.push(num * count_for_num);
    }
  });

  similarity.iter().sum()
}

fn parse_string_into_lists(input_file: &str) -> (Vec<u32>, Vec<u32>) {
  let mut left_side: Vec<u32> = vec![];
  let mut right_side: Vec<u32> = vec![];

  input_file.lines().for_each(|line| {
    let left_and_right: Vec<&str> = line.split_whitespace().collect();

    let [left, right, ..] = &left_and_right[..] else {
      return;
    };

    let left = left.parse::<u32>().unwrap();
    let right = right.parse::<u32>().unwrap();

    left_side.push(left);
    right_side.push(right);
  });

  (left_side, right_side)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(11));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(31));
  }
}
