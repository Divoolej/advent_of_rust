mod day_1;
mod day_2;

use crate::year::Year;

use day_1::Day1;
use day_2::Day2;

pub struct Year2015;

impl Year for Year2015 {
  fn year() -> u16 { 2015 }

  fn solve_day_1(variant: &str) {
    Self::solve_day::<Day1>(variant);
  }

  fn solve_day_2(variant: &str) {
    Self::solve_day::<Day2>(variant);
  }
}
