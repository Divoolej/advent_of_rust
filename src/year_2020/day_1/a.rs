use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

pub fn solve(input_dir: &str) -> String {
  let input = input!(input_dir, "report.txt");
  let mut map = HashSet::new();

  for line in input.lines() {
    let number: u64 = line.parse().unwrap();

    if map.contains(&number) {
      return (number * (2020 - number)).to_string();
    } else {
      map.insert(2020 - number);
    }
  }

  "Couldn't find two numbers that add up to 2020".to_string()
}
