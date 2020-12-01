use std::env;

#[macro_use]
mod macros;

mod year;
mod day;

mod year_2015;
mod year_2016;
mod year_2017;
mod year_2020;

use year::Year;

use year_2015::Year2015;
use year_2016::Year2016;
use year_2017::Year2017;
use year_2020::Year2020;

fn main() {
  let args: Vec<String> = env::args().into_iter().skip(1).collect();
  if let [year, day, variant] = &args[..] {
    solve(year, day, variant);
  } else {
    println!("Error: Invalid arguments, correct usage is: advent <YEAR> <DAY> <VARIANT>");
  }
}

fn solve(year: &str, day: &str, variant: &str) {
  match year {
    "2015" => Year2015::solve(day, variant),
    "2016" => Year2016::solve(day, variant),
    "2017" => Year2017::solve(day, variant),
    "2020" => Year2020::solve(day, variant),
    _ => println!("Error: invalid year: {}", year),
  }
}
