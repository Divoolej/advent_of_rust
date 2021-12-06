mod day_1;
mod day_2;

use crate::year::Year;

use day_1::Day1;
use day_2::Day2;

pub struct Year2021;

impl Year for Year2021 {
  year!(2021);

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
  fn year_2021_day_1() {
    assert_eq!(Year2021::solve_day_1("a"), "1121");
    assert_eq!(Year2021::solve_day_1("b"), "1065");
  }

  #[test]
  fn year_2021_day_2() {
    assert_eq!(Year2021::solve_day_2("a"), "1962940");
    assert_eq!(Year2021::solve_day_2("b"), "1813664422");
  }
}
