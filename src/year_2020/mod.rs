mod day_1;

use crate::year::Year;

use day_1::Day1;

pub struct Year2020;

impl Year for Year2020 {
  year!(2020);

  fn solve_day_1(variant: &str) -> String {
    Self::solve_day::<Day1>(variant)
  }
}
