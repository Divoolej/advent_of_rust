pub fn solve(input_dir: &str) -> String {
  let input = input!(input_dir, "report.txt");
  let measurements = input.lines()
  	.filter_map(|line| line.parse::<usize>().ok());

  let sums = measurements.clone()
    .zip(measurements.clone().skip(1))
    .zip(measurements.skip(2))
    .map(|((first, second), third)| {
      first + second + third
    });

  sums.clone()
    .zip(sums.skip(1))
    .fold(0, |count, (current, next)| {
      if next > current { count + 1 } else { count }
    })
    .to_string()
}
