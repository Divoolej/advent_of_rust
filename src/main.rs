mod year_2015;
mod year_2016;
mod year_2017;

fn main() {
    // 2015
    println!("- Year 2015:");
    println!("| Day 1: ");
    println!("| - A) {}", year_2015::day_1::solve_a());
    println!("| - B) {}", year_2015::day_1::solve_b());
    println!("| Day 2: ");
    println!("| - A) {}", year_2015::day_2::solve_a());
    println!("| - B) {}", year_2015::day_2::solve_b());
    // 2016
    println!("- Year 2016:");
    println!("| Day 1: ");
    println!("| - A) {}", year_2016::day_1::solve_a());
    println!("| - B) {}", year_2016::day_1::solve_b());
    // 2017
    println!("- Year 2017:");
    println!("| Day 1: ");
    println!("| - A) {}", year_2017::day_1::solve_a());
    println!("| - B) {}", year_2017::day_1::solve_b());
    println!("| Day 2: ");
    println!("| - A) {}", year_2017::day_2::solve_a());
    println!("| - B) {}", year_2017::day_2::solve_b());
    println!("| Day 3: ");
    println!("| - A) {}", year_2017::day_3::solve_a());
    println!("| - B) {}", year_2017::day_3::solve_b());
}
