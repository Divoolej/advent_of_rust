pub fn solve(input_dir: &str) -> String {
  let input = input!(input_dir, "course.txt");

  let result = input.lines()
    .map(|line| {
      let mut parts = line.split(" ");
      (
        parts.next().unwrap(),
        parts.next().unwrap().parse::<usize>().unwrap()
      )
    })
    .fold((0, 0), |(x, y), (command, arg)| {
      match command {
        "forward" => (x + arg, y),
        "up" => (x, y - arg),
        "down" => (x, y + arg),
        _ => unreachable!(),
      }
    });

  (result.0 * result.1).to_string()
}
