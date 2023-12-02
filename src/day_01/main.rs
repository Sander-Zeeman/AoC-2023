use common::read_input;

fn part1(input: &str) -> i32 {
  input
    .lines()
    .map(|line| {
      let idx1 = line.find(|c: char| c.is_ascii_digit()).unwrap();
      let idx2 = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
      let chars: Vec<char> = line.chars().collect();
      let a = chars[idx1];
      let b = chars[idx2];
  	  format!("{}{}", a, b).parse::<i32>().unwrap()
    }).sum()
}

use std::collections::HashMap;

fn word_map() -> HashMap<&'static str, usize> {
  let mut words = HashMap::new();
  words.insert("one", 1);
  words.insert("two", 2);
  words.insert("three", 3);
  words.insert("four", 4);
  words.insert("five", 5);
  words.insert("six", 6);
  words.insert("seven", 7);
  words.insert("eight", 8);
  words.insert("nine", 9);
  words
}

fn first_digit(line: &str) -> usize {
  let wm = word_map();
  let pair = wm
    .keys()
    .map(|key| (line.find(key), key))
    .filter(|(idx, _)| idx.is_some())
    .map(|(idx, key)| (idx.unwrap(), key))
    .min_by(|(a, _), (b, _)| a.cmp(b));
  let digit_index = line.find(|c: char| c.is_ascii_digit());

  let chars: Vec<char> = line.chars().collect();
  match (pair, digit_index) {
  	(Some((idx, key)), Some(b)) => if idx < b {*wm.get(key).unwrap()} else {chars[b].to_digit(10).unwrap() as usize},
  	(Some((_, key)), None) => *wm.get(key).unwrap(),
  	(None, Some(b)) => chars[b].to_digit(10).unwrap() as usize,
  	(None, None) => panic!("")
  }
}

fn last_digit(line: &str) -> usize {
  let wm = word_map();
  let pair = wm
    .keys()
    .map(|key| (line.rfind(key), key))
    .filter(|(idx, _)| idx.is_some())
    .map(|(idx, key)| (idx.unwrap(), key))
    .max_by(|(a, _), (b, _)| a.cmp(b));
  let digit_index = line.rfind(|c: char| c.is_ascii_digit());

  let chars: Vec<char> = line.chars().collect();
  match (pair, digit_index) {
  	(Some((idx, key)), Some(b)) => if idx > b {*wm.get(key).unwrap()} else {chars[b].to_digit(10).unwrap() as usize},
  	(Some((_, key)), None) => *wm.get(key).unwrap(),
  	(None, Some(b)) => chars[b].to_digit(10).unwrap() as usize,
  	(None, None) => panic!("")
  }
}

fn part2(input: &str) -> i32 {
  input
    .lines()
    .map(|line| {
      let a = first_digit(line);
      let b = last_digit(line);
  	  format!("{}{}", a, b).parse::<i32>().unwrap()
    }).sum()
}

fn main() {
  println!("Test 1: {}", part1(read_input("./test1.txt").as_str()));
  println!("Test 2: {}", part2(read_input("./test2.txt").as_str()));
  println!("Part 1: {}", part1(read_input("./input1.txt").as_str()));
  println!("Part 2: {}", part2(read_input("./input2.txt").as_str()));
}
