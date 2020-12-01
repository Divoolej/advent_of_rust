pub fn solve(input_dir: &str) -> String {
    let input = input!(input_dir, "captcha.txt");
    let numbers: Vec<i32> = input.split("").filter_map(|c| c.parse().ok()).collect();
    let mut sum = 0;
    for i in 0..(numbers.len() - 1) {
        if numbers[i] == numbers[i + 1] {
            sum += numbers[i + 1];
        }
    }
    if numbers[0] == numbers[numbers.len() - 1] {
        sum += numbers[numbers.len() - 1];
    }
    sum.to_string()
}
