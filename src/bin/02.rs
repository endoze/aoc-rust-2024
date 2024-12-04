advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
  let safe_reports_count = input
    .lines()
    .filter(|line| {
      let report: Vec<u32> = line
        .split_whitespace()
        .filter_map(|num| num.parse::<u32>().ok())
        .collect();

      report_is_valid(&report)
    })
    .count();

  Some(safe_reports_count as u32)
}

#[allow(unused)]
pub fn part_two(input: &str) -> Option<u32> {
  let safe_reports_count = input
    .lines()
    .filter(|line| {
      let report: Vec<u32> = line
        .split_whitespace()
        .filter_map(|num| num.parse::<u32>().ok())
        .collect();

      report_is_dampened_valid(&report)
    })
    .count();

  Some(safe_reports_count as u32)
}

fn report_is_valid(report: &[u32]) -> bool {
  if report.len() == 1 {
    return true;
  }

  let ascending = report.windows(2).all(|win| win[0] < win[1]);
  let descending = report.windows(2).all(|win| win[0] > win[1]);

  if !ascending && !descending {
    return false;
  }

  report
    .windows(2)
    .all(|win| (1..=3).contains(&win[0].abs_diff(win[1])))
}

fn report_is_dampened_valid(report: &[u32]) -> bool {
  if report_is_valid(report) {
    return true;
  }

  for i in 0..report.len() {
    let report = [&report[0..i], &report[i + 1..]].concat();

    if report_is_valid(&report) {
      return true;
    }
  }

  false
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(2));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(4));
  }
}
