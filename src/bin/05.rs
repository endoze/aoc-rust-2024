use std::{cmp::Ordering, collections::HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
  let (orderings, updates) = input.split_once("\n\n").unwrap();

  let orderings: HashSet<(usize, usize)> = orderings
    .lines()
    .map(|line| (line[0..2].parse().unwrap(), line[3..].parse().unwrap()))
    .collect();

  let compare = |x: &usize, y: &usize| !orderings.contains(&(*y, *x));

  let sum: u32 = updates
    .lines()
    .flat_map(|update| {
      let update: Vec<usize> = update.split(",").map(|p| p.parse().unwrap()).collect();

      if update.is_sorted_by(compare) {
        return Some(update[update.len() / 2] as u32);
      }

      None
    })
    .sum();

  Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
  let (orderings, updates) = input.split_once("\n\n").unwrap();

  let orderings: HashSet<(usize, usize)> = orderings
    .lines()
    .map(|line| (line[0..2].parse().unwrap(), line[3..].parse().unwrap()))
    .collect();

  let compare = |x: &usize, y: &usize| {
    let (x, y) = (*x, *y);
    if orderings.contains(&(x, y)) {
      Ordering::Less
    } else if orderings.contains(&(y, x)) {
      Ordering::Greater
    } else {
      Ordering::Equal
    }
  };

  let sum = updates
    .lines()
    .map(|update| {
      let mut update: Vec<usize> = update.split(",").map(|x| x.parse().unwrap()).collect();

      if update.is_sorted_by(|a, b| compare(a, b) != Ordering::Greater) {
        0
      } else {
        update.sort_by(compare);
        update[update.len() / 2] as u32
      }
    })
    .sum();

  Some(sum)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(143));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(123));
  }
}
