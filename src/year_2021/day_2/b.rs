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
    .fold((0, 0, 0), |(aim, x, y), (command, arg)| {
      match command {
        "forward" => (aim, x + arg, y + aim * arg),
        "up" => (aim - arg, x, y),
        "down" => (aim + arg, x, y),
        _ => unreachable!(),
      }
    });

  (result.1 * result.2).to_string()
}
