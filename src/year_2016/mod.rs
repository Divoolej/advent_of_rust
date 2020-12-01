mod day_1;

use crate::year::Year;

use day_1::Day1;

pub struct Year2016;

impl Year for Year2016 {
  year!(2016);

  fn solve_day_1(variant: &str) -> String {
    Self::solve_day::<Day1>(variant)
  }
}
