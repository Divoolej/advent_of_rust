macro_rules! year {
  ($n:expr) => {
    fn year() -> u16 { $n }
  };
}

macro_rules! day {
  ($n:expr) => {
    fn day() -> u8 { $n }
  };
}

macro_rules! solve_a {
  () => {
    fn solve_a(year: u16) -> String {
      a::solve(Self::input_dir(year).as_str())
    }
  };
}

macro_rules! solve_b {
  () => (
    fn solve_b(year: u16) -> String {
      b::solve(Self::input_dir(year).as_str())
    }
  )
}

macro_rules! input {
  ($dir:expr, $name:expr) => {
    {
      use std::fs::File;
      use std::io::Read;
      let mut file = File::open(format!("{}/{}", $dir, $name))
        .expect(format!("{}/{} not found", $dir, $name).as_str());
      let mut contents = String::new();
      file.read_to_string(&mut contents)
          .expect(format!("Error reading {}/{}", $dir, $name).as_str());
      contents
    }
  };
}
