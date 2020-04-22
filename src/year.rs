use crate::day::Day;

pub trait Year {
  fn year() -> u16;

  fn not_implemented(day: u8) {
    println!("Warning: Day {} of {} not implemented yet", day, Self::year());
  }

  fn solve_day<T: Day>(variant: &str) {
    T::solve(variant, Self::year());
  }

  fn solve_day_1(_variant: &str)  { Self::not_implemented(1);  }
  fn solve_day_2(_variant: &str)  { Self::not_implemented(2);  }
  fn solve_day_3(_variant: &str)  { Self::not_implemented(3);  }
  fn solve_day_4(_variant: &str)  { Self::not_implemented(4);  }
  fn solve_day_5(_variant: &str)  { Self::not_implemented(5);  }
  fn solve_day_6(_variant: &str)  { Self::not_implemented(6);  }
  fn solve_day_7(_variant: &str)  { Self::not_implemented(7);  }
  fn solve_day_8(_variant: &str)  { Self::not_implemented(8);  }
  fn solve_day_9(_variant: &str)  { Self::not_implemented(9);  }
  fn solve_day_10(_variant: &str) { Self::not_implemented(10); }
  fn solve_day_11(_variant: &str) { Self::not_implemented(11); }
  fn solve_day_12(_variant: &str) { Self::not_implemented(12); }
  fn solve_day_13(_variant: &str) { Self::not_implemented(13); }
  fn solve_day_14(_variant: &str) { Self::not_implemented(14); }
  fn solve_day_15(_variant: &str) { Self::not_implemented(15); }
  fn solve_day_16(_variant: &str) { Self::not_implemented(16); }
  fn solve_day_17(_variant: &str) { Self::not_implemented(17); }
  fn solve_day_18(_variant: &str) { Self::not_implemented(18); }
  fn solve_day_19(_variant: &str) { Self::not_implemented(19); }
  fn solve_day_20(_variant: &str) { Self::not_implemented(20); }
  fn solve_day_21(_variant: &str) { Self::not_implemented(21); }
  fn solve_day_22(_variant: &str) { Self::not_implemented(22); }
  fn solve_day_23(_variant: &str) { Self::not_implemented(23); }
  fn solve_day_24(_variant: &str) { Self::not_implemented(24); }
  fn solve_day_25(_variant: &str) { Self::not_implemented(25); }
  fn solve_day_26(_variant: &str) { Self::not_implemented(26); }
  fn solve_day_27(_variant: &str) { Self::not_implemented(27); }
  fn solve_day_28(_variant: &str) { Self::not_implemented(28); }
  fn solve_day_29(_variant: &str) { Self::not_implemented(29); }
  fn solve_day_30(_variant: &str) { Self::not_implemented(30); }
  fn solve_day_31(_variant: &str) { Self::not_implemented(31); }

  fn solve(day: &str, variant: &str) {
    match day.parse::<u64>() {
      Ok(1) => Self::solve_day_1(variant),
      Ok(2) => Self::solve_day_2(variant),
      Ok(3) => Self::solve_day_3(variant),
      Ok(4) => Self::solve_day_4(variant),
      Ok(5) => Self::solve_day_5(variant),
      Ok(6) => Self::solve_day_6(variant),
      Ok(7) => Self::solve_day_7(variant),
      Ok(8) => Self::solve_day_8(variant),
      Ok(9) => Self::solve_day_9(variant),
      Ok(10) => Self::solve_day_10(variant),
      Ok(11) => Self::solve_day_11(variant),
      Ok(12) => Self::solve_day_12(variant),
      Ok(13) => Self::solve_day_13(variant),
      Ok(14) => Self::solve_day_14(variant),
      Ok(15) => Self::solve_day_15(variant),
      Ok(16) => Self::solve_day_16(variant),
      Ok(17) => Self::solve_day_17(variant),
      Ok(18) => Self::solve_day_18(variant),
      Ok(19) => Self::solve_day_19(variant),
      Ok(20) => Self::solve_day_20(variant),
      Ok(21) => Self::solve_day_21(variant),
      Ok(22) => Self::solve_day_22(variant),
      Ok(23) => Self::solve_day_23(variant),
      Ok(24) => Self::solve_day_24(variant),
      Ok(25) => Self::solve_day_25(variant),
      Ok(26) => Self::solve_day_26(variant),
      Ok(27) => Self::solve_day_27(variant),
      Ok(28) => Self::solve_day_28(variant),
      Ok(29) => Self::solve_day_29(variant),
      Ok(30) => Self::solve_day_30(variant),
      Ok(31) => Self::solve_day_31(variant),
      Ok(_) | Err(_) => println!("Error: Invalid day: {}", day),
    }
  }
}
