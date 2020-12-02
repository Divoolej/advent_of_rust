fn is_nice(string: Vec<char>) -> bool {
  let mut first_condition = false;
  let mut second_condition = false;

  for i in 0..(string.len() - 2) {
    if !second_condition {
      if string[i] == string[i + 2] {
        if first_condition {
          return true;
        } else {
          second_condition = true;
        }
      }
    }

    if !first_condition {
      for j in (i + 2)..(string.len() - 2) {
        if string[i..(i+2)] == string[j..(j+2)] {
          if second_condition {
            return true;
          } else {
            first_condition = true;
          }
        }
      }
    }
  }

  first_condition && second_condition
}

pub fn solve(input_dir: &str) -> String {
  let input = "xxyxx"; //input!(input_dir, "strings.txt");
  let mut count = 0;

  for line in input.lines() {
    if is_nice(line.trim().chars().collect()) {
      count += 1;
    }
  }

  count.to_string()
}
