use std::collections::HashSet;

fn move_santa(x: &mut i8, y: &mut i8, direction: char) {
  match direction {
    '^' => *y += 1,
    '>' => *x += 1,
    '<' => *x -= 1,
    'v' => *y -= 1,
    _ => (),
  }
}

pub fn solve(input_dir: &str) -> String {
  let input = input!(input_dir, "directions.txt");

  let mut map = HashSet::new();
  let (mut santa_x, mut santa_y) = (0i8, 0i8);
  let (mut robo_x, mut robo_y) = (0i8, 0i8);
  map.insert((santa_x, santa_y));
  let mut santas_turn = true;
  for direction in input.chars() {
    if santas_turn {
      move_santa(&mut santa_x, &mut santa_y, direction);
      map.insert((santa_x, santa_y));
      santas_turn = false;
    } else {
      move_santa(&mut robo_x, &mut robo_y, direction);
      map.insert((robo_x, robo_y));
      santas_turn = true;
    }
  }
  map.len().to_string()
}
