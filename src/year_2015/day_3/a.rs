use std::collections::HashSet;

pub fn solve(input_dir: &str) -> String {
  let input = input!(input_dir, "directions.txt");

  let mut map = HashSet::new();
  let (mut x, mut y) = (0, 0);
  map.insert((x, y));
  for direction in input.chars() {
    match direction {
      '^' => y += 1,
      '>' => x += 1,
      '<' => x -= 1,
      'v' => y -= 1,
      _ => continue,
    }
    map.insert((x, y));
  }
  map.len().to_string()
}
