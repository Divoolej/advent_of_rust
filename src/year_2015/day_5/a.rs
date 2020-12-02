fn is_vowel(c: char) -> bool {
  match c.to_ascii_lowercase() {
    'a' | 'e' | 'i' | 'o' | 'u' => true,
    _ => false,
  }
}

fn is_forbidden(s: &str) -> bool {
  match s {
    "ab" | "cd" | "pq" | "xy" => true,
    _ => false,
  }
}

fn is_nice(string: &str) -> bool {
  let vowels = string
    .chars()
    .fold(0, |acc, curr| if is_vowel(curr) { acc + 1 } else { acc });

  let mut has_twice_in_a_row = false;

  for w in string.chars().collect::<Vec<char>>().windows(2) {
    if is_forbidden(&w.iter().collect::<String>()) { return false; }
    if w[0] == w[1] { has_twice_in_a_row = true; }
  }

  vowels >= 3 && has_twice_in_a_row
}

pub fn solve(input_dir: &str) -> String {
  let input = input!(input_dir, "strings.txt");
  let mut count = 0;

  for line in input.lines() {
    if is_nice(line.trim()) {
      count += 1;
    }
  }

  count.to_string()
}
