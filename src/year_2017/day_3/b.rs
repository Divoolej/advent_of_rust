use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn move_impossible(pos: &(i32, i32), level: i32, direction: &char) -> bool {
    match *direction {
        'N' => { if pos.1 == level { true } else { false } },
        'E' => { if pos.0 == level { true } else { false } },
        'S' => { if pos.1 == -level { true } else { false } },
        'W' => { if pos.0 == -level { true } else { false } },
        _ => false
    }
}

fn rotate(direction: &char) -> char {
    match *direction {
        'N' => 'W',
        'E' => 'N',
        'S' => 'E',
        'W' => 'S',
        _ => *direction
    }
}

fn move_in_direction(pos: &(i32, i32), direction: &char) -> (i32, i32) {
    match *direction {
        'N' => (pos.0, pos.1 + 1),
        'E' => (pos.0 + 1, pos.1),
        'S' => (pos.0, pos.1 - 1),
        'W' => (pos.0 - 1, pos.1),
        _ => *pos
    }
}

fn sum_adjacent(spiral: &HashMap<(i32, i32), i32>, pos: &(i32, i32)) -> i32 {
  let mut sum = 0;
  sum += match spiral.get(&(pos.0 + 1, pos.1    )) { Some(n) => *n, None => 0 };
  sum += match spiral.get(&(pos.0 + 1, pos.1 + 1)) { Some(n) => *n, None => 0 };
  sum += match spiral.get(&(pos.0,     pos.1 + 1)) { Some(n) => *n, None => 0 };
  sum += match spiral.get(&(pos.0 - 1, pos.1 + 1)) { Some(n) => *n, None => 0 };
  sum += match spiral.get(&(pos.0 - 1, pos.1    )) { Some(n) => *n, None => 0 };
  sum += match spiral.get(&(pos.0 - 1, pos.1 - 1)) { Some(n) => *n, None => 0 };
  sum += match spiral.get(&(pos.0    , pos.1 - 1)) { Some(n) => *n, None => 0 };
  sum += match spiral.get(&(pos.0 + 1, pos.1 - 1)) { Some(n) => *n, None => 0 };
  sum
}

pub fn solve() -> String {
    let mut file = File::open("inputs/2017/3/input.txt").expect("inputs/2017/3/input.txt not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading inputs/2017/3/input.txt");
    let target: i32 = contents.parse().expect("Error parsing inputs/2017/3/input.txt");
    let mut spiral = HashMap::new();
    spiral.insert((0, 0), 1);
    let mut value;
    let mut n = 1;
    let mut pos = (1, 0);
    let mut direction = 'N';
    loop {
      value = sum_adjacent(&spiral, &pos);
      if value > target { break; }
      spiral.insert(pos, value);
      if move_impossible(&pos, n, &direction) { direction = rotate(&direction); }
      if pos == (n, -n) {
        n += 1;
        pos = (n, -n + 1);
      } else {
        pos = move_in_direction(&pos, &direction)
      }
    }
    value.to_string()
}
