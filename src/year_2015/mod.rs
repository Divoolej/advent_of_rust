mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

use crate::year::Year;

use day_1::Day1;
use day_2::Day2;
use day_3::Day3;
use day_4::Day4;
use day_5::Day5;

pub struct Year2015;

impl Year for Year2015 {
  year!(2015);

  fn solve_day_1(variant: &str) -> String {
    Self::solve_day::<Day1>(variant)
  }

  fn solve_day_2(variant: &str) -> String {
    Self::solve_day::<Day2>(variant)
  }

  fn solve_day_3(variant: &str) -> String {
    Self::solve_day::<Day3>(variant)
  }

  fn solve_day_4(variant: &str) -> String {
    Self::solve_day::<Day4>(variant)
  }

  fn solve_day_5(variant: &str) -> String {
    Self::solve_day::<Day5>(variant)
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

  #[test]
  fn year_2015_day_3() {
    assert_eq!(Year2015::solve_day_3("a"), "2081");
    assert_eq!(Year2015::solve_day_3("b"), "2341");
  }

  #[test]
  fn year_2015_day_4() {
    assert_eq!(Year2015::solve_day_4("a"), "282749");
    assert_eq!(Year2015::solve_day_4("b"), "9962624");
  }

  #[test]
  fn year_2015_day_5() {
    assert_eq!(Year2015::solve_day_5("a"), "236");
    assert_eq!(Year2015::solve_day_5("b"), "");
  }
}
