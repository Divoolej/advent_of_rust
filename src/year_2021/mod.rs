mod day_1;

use crate::year::Year;

use day_1::Day1;

pub struct Year2021;

impl Year for Year2021 {
  year!(2021);

  fn solve_day_1(variant: &str) -> String {
    Self::solve_day::<Day1>(variant)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn year_2021_day_1() {
    assert_eq!(Year2021::solve_day_1("a"), "1121");
  }
}
