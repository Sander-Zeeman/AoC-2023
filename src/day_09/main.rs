use std::iter::zip;
use common::read_input;

fn all_zeroes(line: &Vec<i32>) -> bool {
  for num in line {
    if *num != 0 {
      return false;
    }
  }

  true
}

fn predict1(line: &Vec<i32>) -> i32 {
  if line.len() < 2 {
    panic!();
  }

  if all_zeroes(line) {
    return 0;
  }

  let mut diff: Vec<i32> = Vec::new();
  let first = &line[0..line.len()-1];
  let tail = &line[1..line.len()];
  for (a, b) in zip(first, tail) {
    diff.push(b - a);
  }

  predict1(&diff) + line.last().unwrap()
}

fn predict2(line: &Vec<i32>) -> i32 {
  if line.len() < 2 {
    panic!();
  }

  if all_zeroes(line) {
    return 0;
  }

  let mut diff: Vec<i32> = Vec::new();
  let first = &line[0..line.len()-1];
  let tail = &line[1..line.len()];
  for (a, b) in zip(first, tail) {
    diff.push(b - a);
  }

  line.first().unwrap() - predict2(&diff)
}

fn part1(input: &str) -> i32 {
  let lines: Vec<Vec<i32>> = input.lines().map(|line| {
    line.split(' ').map(|num| num.parse().unwrap()).collect()
  }).collect();

  let mut total: i32 = 0;
  for line in lines {
    total += predict1(&line);
  }

  total
}

fn part2(input: &str) -> i32 {
  let lines: Vec<Vec<i32>> = input.lines().map(|line| {
    line.split(' ').map(|num| num.parse().unwrap()).collect()
  }).collect();

  let mut total: i32 = 0;
  for line in lines {
    total += predict2(&line);
  }

  total
}

fn main() {
  println!("Test 1: {}", part1(read_input("./test.txt").as_str()));
  println!("Test 2: {}", part2(read_input("./test.txt").as_str()));
  println!("Part 1: {}", part1(read_input("./input.txt").as_str()));
  println!("Part 2: {}", part2(read_input("./input.txt").as_str()));
}
