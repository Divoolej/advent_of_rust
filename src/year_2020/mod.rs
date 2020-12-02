mod day_1;
mod day_2;

use crate::year::Year;

use day_1::Day1;
use day_2::Day2;

pub struct Year2020;

impl Year for Year2020 {
  year!(2020);

  fn solve_day_1(variant: &str) -> String {
    Self::solve_day::<Day1>(variant)
  }

  fn solve_day_2(variant: &str) -> String {
    Self::solve_day::<Day2>(variant)
  }
}
