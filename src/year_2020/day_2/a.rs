fn valid_password(min: usize, max: usize, letter: String, password: &str) -> bool {
  let count = password.split(&letter).collect::<Vec<_>>().len() - 1;
  count >= min && count <= max
}

pub fn solve(input_dir: &str) -> String {
  let input = input!(input_dir, "passwords.txt");

  let mut count = 0;

  for line in input.lines() {
    if let [policy, letter, password] = line.split(' ').collect::<Vec<_>>()[..] {
      if let [min, max] = policy.split('-')
                                .map(|el| el.parse::<usize>().unwrap())
                                .collect::<Vec<_>>()[..] {
        if valid_password(min, max, letter.replace(':', ""), password) { count += 1; }
      }
    }
  }
  count.to_string()
}
