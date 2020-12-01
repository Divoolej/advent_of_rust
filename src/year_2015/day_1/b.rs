pub fn solve(input_dir: &str) -> String {
    let input = input!(input_dir, "input.txt");
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        }
        if c == ')' {
            floor -= 1;
        }
        if floor == -1 {
            return (i + 1).to_string();
        }
    }
    '0'.to_string()
}
