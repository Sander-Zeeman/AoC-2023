use common::read_input;

fn part1(input: &str) -> i32 {
  let mut total = 0;
  for (idx, line) in input.lines().enumerate() {
    let draws = line.split(':')
      .last()
      .unwrap()
      .split(';');

    let mut max_r = i32::MIN;
    let mut max_g = i32::MIN;
    let mut max_b = i32::MIN;

    for draw in draws {
      let singles = draw
        .split(',')
        .map(|elem| elem.trim());

      for single in singles {
        let parts: Vec<&str> = single.split(' ').collect();
        match parts[..] {
          [r, "red"] => max_r = max_r.max(r.parse::<i32>().unwrap()),
          [g, "green"] => max_g = max_g.max(g.parse::<i32>().unwrap()),
          [b, "blue"] => max_b = max_b.max(b.parse::<i32>().unwrap()),
          _ => {}
        };
      }
    }

    if max_r <= 12 && max_g <= 13 && max_b <= 14 {
      total += idx + 1;
    }
  }
  total as i32
}

fn part2(input: &str) -> i32 {
  let mut total = 0;
  for line in input.lines() {
    let draws = line.split(':')
      .last()
      .unwrap()
      .split(';');

    let mut max_r = i32::MIN;
    let mut max_g = i32::MIN;
    let mut max_b = i32::MIN;

    for draw in draws {
      let singles = draw
        .split(',')
        .map(|elem| elem.trim());

      for single in singles {
        let parts: Vec<&str> = single.split(' ').collect();
        match parts[..] {
          [r, "red"] => max_r = max_r.max(r.parse::<i32>().unwrap()),
          [g, "green"] => max_g = max_g.max(g.parse::<i32>().unwrap()),
          [b, "blue"] => max_b = max_b.max(b.parse::<i32>().unwrap()),
          _ => {}
        };
      }
    }

    total += max_r * max_g * max_b;
  }
  total as i32
}

fn main() {
  println!("Test 1: {}", part1(read_input("./test1.txt").as_str()));
  println!("Test 2: {}", part2(read_input("./test2.txt").as_str()));
  println!("Part 1: {}", part1(read_input("./input1.txt").as_str()));
  println!("Part 2: {}", part2(read_input("./input2.txt").as_str()));
}
