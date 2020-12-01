use std::fs::File;
use std::io::Read;

pub fn solve(input_dir: &str) -> String {
  let input = input!(input_dir, "report.txt");

  let mut numbers: Vec<u64> = input
    .lines()
    .map(|line| line.parse().unwrap())
    .collect();

  numbers.sort_unstable();

  for (idx_max, &max) in numbers.iter().enumerate().rev() {
    if max >= 2020 { break; }
    for (idx_min, min) in numbers[..idx_max].iter().enumerate() {
      if min + max >= 2020 { break; }
      for middle in numbers[(idx_min + 1)..idx_max].iter() {
        if min + middle + max == 2020 { return (min * middle * max).to_string(); }
        if min + middle + max > 2020 { break; }
      }
    }
  }

  "Couldn't find three numbers that add up to 2020".to_string()
}
