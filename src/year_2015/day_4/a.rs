pub fn solve(input_dir: &str) -> String {
  let input = input!(input_dir, "key.txt").trim().to_string();
  let mut i = 1;
  loop {
    if let [a, b, c] = md5::compute(format!("{}{}", input, i))[..3] {
      if a == 0 && b == 0 && c & 0xf0 == 0 { return i.to_string(); }
    }
    i += 1;
  }
}
