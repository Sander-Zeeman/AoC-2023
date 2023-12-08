use std::collections::HashMap;

use common::read_input;

type Ident<'a> = &'a str;

fn part1(input: &str) -> i32 {
  let mut lines = input.lines();
  let order: Vec<char> = lines.next().unwrap().chars().collect();
  let _ = lines.next();
  let mut hm: HashMap<Ident, (Ident, Ident)> = HashMap::new();
  for line in lines {
    let split: Vec<&str> = line.split(" = ").collect();
    let key = split[0];
    let pair: Vec<Ident> = split[1][1..9].split(", ").collect();
    let left = pair[0];
    let right = pair[1];
    hm.insert(key, (left, right));
  }

  let mut pos: Ident = "AAA";
  let mut i = 0;
  while pos != "ZZZ" {
    pos = match order[i % order.len()] {
      'L' => hm.get(&pos).unwrap().0,
      'R' => hm.get(&pos).unwrap().1,
      _ => panic!()
    };

    i += 1;
  }

  i as i32
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}


fn part2(input: &str) -> usize {
  let mut lines = input.lines();
  let order: Vec<char> = lines.next().unwrap().chars().collect();
  let _ = lines.next();
  let mut hm: HashMap<Ident, (Ident, Ident)> = HashMap::new();
  for line in lines {
    let split: Vec<&str> = line.split(" = ").collect();
    let key = split[0];
    let pair: Vec<Ident> = split[1][1..9].split(", ").collect();
    let left = pair[0];
    let right = pair[1];
    hm.insert(key, (left, right));
  }

  let positions = hm.keys().cloned().filter(|key| key.ends_with('A'));
  let lengths = positions.map(|mut pos| {
    let mut i = 0;
    while !pos.ends_with('Z') {
      pos = match order[i % order.len()] {
        'L' => hm.get(pos).unwrap().0,
        'R' => hm.get(pos).unwrap().1,
        _ => panic!()
      };
      i += 1;
    }
    i
  });

  lengths.fold(1, |a, b| lcm(a, b))
}

fn main() {
  println!("Test 1: {}", part1(read_input("./test1.txt").as_str()));
  println!("Test 2: {}", part2(read_input("./test2.txt").as_str()));
  println!("Part 1: {}", part1(read_input("./input.txt").as_str()));
  println!("Part 2: {}", part2(read_input("./input.txt").as_str()));
}
