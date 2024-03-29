pub fn solve(input_dir: &str) -> String {
  let input = input!(input_dir, "report.txt");
  let measurements = input
  	.lines()
  	.filter_map(|line| line.parse::<usize>().ok());

  measurements
    .clone()
    .zip(measurements.skip(1))
    .fold(0, |count, (current, next)| {
      if next > current { count + 1 } else { count }
    })
    .to_string()
}
