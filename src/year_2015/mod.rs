mod day_1;
mod day_2;

use crate::year::Year;

use day_1::Day1;
use day_2::Day2;

pub struct Year2015;

impl Year for Year2015 {
  year!(2015);

  fn solve_day_1(variant: &str) -> String {
    Self::solve_day::<Day1>(variant)
  }

  fn solve_day_2(variant: &str) -> String {
    Self::solve_day::<Day2>(variant)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn year_2015_day_1() {
    assert_eq!(Year2015::solve_day_1("a"), "280");
    assert_eq!(Year2015::solve_day_1("b"), "1797");
  }

  #[test]
  fn year_2015_day_2() {
    assert_eq!(Year2015::solve_day_2("a"), "1588178");
    assert_eq!(Year2015::solve_day_2("b"), "3783758");
  }
}
