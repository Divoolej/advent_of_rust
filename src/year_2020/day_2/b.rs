fn valid_password(idx1: usize, idx2: usize, letter: String, password: &str) -> bool {
  let char1 = password.chars().nth(idx1 - 1).unwrap().to_string();
  let char2 = password.chars().nth(idx2 - 1).unwrap().to_string();
  char1 == letter && char2 != letter
    || char1 != letter && char2 == letter
}

pub fn solve(input_dir: &str) -> String {
  let input = input!(input_dir, "passwords.txt");

  let mut count = 0;

  for line in input.lines() {
    if let [policy, letter, password] = line.split(' ').collect::<Vec<_>>()[..] {
      if let [idx1, idx2] = policy.split('-')
                                .map(|el| el.parse::<usize>().unwrap())
                                .collect::<Vec<_>>()[..] {
        if valid_password(idx1, idx2, letter.replace(':', ""), password) { count += 1; }
      }
    }
  }
  count.to_string()
}
