use std::fs::File;
use std::io::Read;

fn calculate_needed_paper(dimensions: &Vec<i32>) -> i32 {
  let area = 2*dimensions[0]*dimensions[1] + 2*dimensions[0]*dimensions[2] + 2*dimensions[1]*dimensions[2];
  let smallest_side_area = dimensions[0] * dimensions[1];
  area + smallest_side_area
}

pub fn solve() -> String {
    let mut file = File::open("inputs/2015/2/dimensions.txt").expect("inputs/2015/2/dimensions.txt not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading inputs/2015/2/dimensions.txt");
    let dimensions_list: Vec<&str> = contents.split("\n").collect();
    let mut sum = 0;
    for dimensions_string in dimensions_list {
      let mut dimensions: Vec<i32> = dimensions_string.split("x").filter_map( |d| d.parse().ok() ).collect();
      dimensions.sort();
      sum += calculate_needed_paper(&dimensions);
    }
    sum.to_string()
}
