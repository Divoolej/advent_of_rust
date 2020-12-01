fn max(n: i32) -> i32 {
    if n <= 0 {
        return 1;
    }
    max(n - 1) + 8 * n
}

fn min(n: i32) -> i32 {
    if n <= 0 {
        return 1;
    }
    max(n - 1) + 1
}

fn level_for_target(target: &i32) -> i32 {
    let mut n = 0;
    loop {
        let max_value = max(n);
        if max_value >= *target {
            break;
        }
        n += 1;
    }
    n
}

fn move_impossible(pos: &(i32, i32), level: i32, direction: &char) -> bool {
    match *direction {
        'N' => {
            if pos.1 == level {
                true
            } else {
                false
            }
        }
        'E' => {
            if pos.0 == level {
                true
            } else {
                false
            }
        }
        'S' => {
            if pos.1 == -level {
                true
            } else {
                false
            }
        }
        'W' => {
            if pos.0 == -level {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn move_in_direction(pos: &(i32, i32), direction: &char) -> (i32, i32) {
    match *direction {
        'N' => (pos.0, pos.1 + 1),
        'E' => (pos.0 + 1, pos.1),
        'S' => (pos.0, pos.1 - 1),
        'W' => (pos.0 - 1, pos.1),
        _ => *pos,
    }
}

fn rotate(direction: &char) -> char {
    match *direction {
        'N' => 'W',
        'E' => 'N',
        'S' => 'E',
        'W' => 'S',
        _ => *direction,
    }
}

fn find_target_pos(start_pos: (i32, i32), level: i32, target: i32) -> (i32, i32) {
    let mut direction = 'N';
    let mut pos = start_pos;
    let mut value = min(level);
    while value < target {
        if move_impossible(&pos, level, &direction) {
            direction = rotate(&direction);
        }
        pos = move_in_direction(&pos, &direction);
        value += 1;
    }
    pos
}

pub fn solve(input_dir: &str) -> String {
    let input = input!(input_dir, "input.txt");
    let target: i32 = input
        .parse()
        .expect("Error parsing inputs/2017/3/input.txt");
    let n = level_for_target(&target);
    let pos = find_target_pos((n, -n + 1), n, target);
    (pos.0.abs() + pos.1.abs()).to_string()
}
