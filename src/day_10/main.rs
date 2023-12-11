use common::read_input;
use std::collections::VecDeque;

#[derive(PartialEq, Copy, Clone)]
enum Dir {
  North,
  East,
  South,
  West,
  Start
}

#[derive(PartialEq, Copy, Clone)]
struct Pipe(Dir, Dir, bool);

fn connect(pipe: Option<Pipe>, dir: Dir) -> bool {
  if pipe.is_none() {
    return false;
  }

  let pipe = pipe.unwrap();
  let goal = match dir {
    Dir::North => Dir::South,
    Dir::East => Dir::West,
    Dir::South => Dir::North,
    Dir::West => Dir::East,
    _ => panic!()
  };
  pipe.0 == goal || pipe.1 == goal
}

fn part1(input: &str) -> i32 {
  let mut grid: Vec<Vec<Option<Pipe>>> = Vec::new();
  let mut start_x = 0;
  let mut start_y = 0;
  for (i, line) in input.lines().enumerate() {
    grid.push(Vec::new());
    for (j, c) in line.chars().enumerate() {
      let last_entry: &mut Vec<Option<Pipe>> = &mut grid[i];
      last_entry.push(match c {
        '|' => Some(Pipe(Dir::North, Dir::South, false)),
        '-' => Some(Pipe(Dir::West, Dir::East, false)),
        'L' => Some(Pipe(Dir::North, Dir::East, false)),
        'J' => Some(Pipe(Dir::North, Dir::West, false)),
        '7' => Some(Pipe(Dir::West, Dir::South, false)),
        'F' => Some(Pipe(Dir::East, Dir::South, false)),
        'S' => {
          start_x = j;
          start_y = i;
          Some(Pipe(Dir::Start, Dir::Start, false))
        },
        '.' => None,
        _ => panic!()
      });
    }
  }

  let mut conns: Vec<Dir> = Vec::new();
  let y_len = grid.len();
  let x_len = grid[0].len();

  if start_y > 0 && connect(grid[start_y-1][start_x], Dir::North) {
      conns.push(Dir::North);
  }
  if start_x > 0 && connect(grid[start_y][start_x-1], Dir::West) {
    conns.push(Dir::West);
  }
  if start_y < y_len-1 && connect(grid[start_y+1][start_x], Dir::South) {
    conns.push(Dir::South);
  }
  if start_x < x_len-1 && connect(grid[start_y][start_x+1], Dir::East) {
    conns.push(Dir::East);
  }

  grid[start_y][start_x] = Some(Pipe(conns[0].clone(), conns[1].clone(), false));

  let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
  queue.push_back((start_y, start_x));

  while let Some((y, x)) = queue.pop_front() {
    let pipe = grid[y][x].unwrap();
    if pipe.2 {
      continue;
    }

    grid[y][x] = Some(Pipe(pipe.0, pipe.1, true));
    if y > 0 && connect(grid[y-1][x], Dir::North) {
      queue.push_back((y-1, x));
    }
    if x > 0 && connect(grid[y][x-1], Dir::West) {
      queue.push_back((y, x-1));
    }
    if y < y_len-1 && connect(grid[y+1][x], Dir::South) {
      queue.push_back((y+1, x));
    }
    if x < x_len-1 && connect(grid[y][x+1], Dir::East) {
      queue.push_back((y, x+1));
    }
  }

  0
}

fn _part2(_input: &str) -> i32 {
  return 0;
}

fn main() {
  println!("Test 1: {}", part1(read_input("./test.txt").as_str()));
  // println!("Test 2: {}", part2(read_input("./test.txt").as_str()));
  // println!("Part 1: {}", part1(read_input("./input.txt").as_str()));
  // println!("Part 2: {}", part2(read_input("./input.txt").as_str()));
}
