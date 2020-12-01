mod day_1;
mod day_2;
mod day_3;

use crate::year::Year;

use day_1::Day1;
use day_2::Day2;
use day_3::Day3;

pub struct Year2017;

impl Year for Year2017 {
  year!(2017);

  fn solve_day_1(variant: &str) -> String {
    Self::solve_day::<Day1>(variant)
  }

  fn solve_day_2(variant: &str) -> String {
    Self::solve_day::<Day2>(variant)
  }

  fn solve_day_3(variant: &str) -> String {
    Self::solve_day::<Day3>(variant)
  }
}
