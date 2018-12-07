use std::fs::File;
use std::io::Read;

fn rotate(direction: char, rotation: &str) -> char {
    match direction {
        'N' => match rotation {
            "R" => 'E',
            "L" => 'W',
            _ => 'N',
        },
        'E' => match rotation {
            "R" => 'S',
            "L" => 'N',
            _ => 'E',
        },
        'S' => match rotation {
            "R" => 'W',
            "L" => 'E',
            _ => 'S',
        },
        'W' => match rotation {
            "R" => 'N',
            "L" => 'S',
            _ => 'W',
        },
        _ => 'N',
    }
}

fn translate(x: i32, y: i32, direction: char, steps: i32) -> (i32, i32) {
    match direction {
        'N' => (x, y + steps),
        'E' => (x + steps, y),
        'S' => (x, y - steps),
        'W' => (x - steps, y),
        _ => (x, y),
    }
}

pub fn solve() -> String {
    let mut file =
        File::open("inputs/2016/1/directions.txt").expect("inputs/2016/1/directions.txt not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading inputs/2016/1/directions.txt");
    let mut pos = (0, 0);
    let mut current_direction = 'N';
    let directions: Vec<String> = contents.split(", ").map(|s| s.to_string()).collect();
    for instruction in directions {
        let rotation = &instruction[..1];
        let steps = (&instruction[1..])
            .parse()
            .expect("Error parsing directions");
        current_direction = rotate(current_direction, &rotation);
        pos = translate(pos.0, pos.1, current_direction, steps);
    }
    (pos.0.abs() + pos.1.abs()).to_string()
}
