fn calculate_needed_ribbon(dimensions: &Vec<i32>) -> i32 {
    let shortest_perimeter = 2 * dimensions[0] + 2 * dimensions[1];
    let volume = dimensions[0] * dimensions[1] * dimensions[2];

    shortest_perimeter + volume
}

pub fn solve(input_dir: &str) -> String {
    let input = input!(input_dir, "dimensions.txt");
    let dimensions_list: Vec<&str> = input.split("\n").collect();
    let mut sum = 0;
    for dimensions_string in dimensions_list {
        let mut dimensions: Vec<i32> = dimensions_string
            .split("x")
            .filter_map(|d| d.parse().ok())
            .collect();
        dimensions.sort();
        sum += calculate_needed_ribbon(&dimensions);
    }
    sum.to_string()
}
