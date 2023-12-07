use std::cmp::Ordering;
use std::collections::HashMap;
use common::read_input;

type HandKind = u32;
type CardKind = u32;
type Bet = u32;

fn part1(input: &str) -> i32 {
  let mut parsed: Vec<(Vec<CardKind>, HandKind, Bet)> = input.lines().map(|line| line.split(' ')).map(|mut split_line| {
    let hand = split_line.next().unwrap();
    let bet = split_line.next().unwrap();
    let parsed_hand: Vec<CardKind> = hand.chars().map(|card| {
      match card {
        '2'..='9' => card.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!()
      }
    }).collect();

    let mut hm = HashMap::new();
    for c in hand.chars() {
      if hm.contains_key(&c) {
        let _ = hm.insert(c, hm.get(&c).unwrap() + 1);
      } else {
        hm.insert(c, 1);
      }
    }

    let hand_kind = match hm.len() {
      5 => 1,
      4 => 2,
      3 => match hm.values().max().unwrap() {
        2 => 3,
        3 => 4,
        _ => panic!()
      },
      2 => match hm.values().max().unwrap() {
        3 => 5,
        4 => 6,
        _ => panic!()
      },
      1 => 7,
      _ => panic!()
    };
    
    let parsed_bet = bet.parse::<u32>().unwrap();

    (parsed_hand, hand_kind, parsed_bet)
  }).collect();

  parsed.sort_by(|(cards_a, hand_a, _), (cards_b, hand_b, _)| {
    let hand_compare = hand_a.cmp(&hand_b);
    if hand_compare != Ordering::Equal {
      return hand_compare;
    }

    for (card_a, card_b) in cards_a.iter().zip(cards_b.iter()) {
      let card_compare = card_a.cmp(&card_b);
      if card_compare != Ordering::Equal {
        return card_compare;
      }
    }

    panic!();
  });

  let mut total = 0;
  for (i, (_, _, bet)) in parsed.iter().enumerate() {
    total += (i as u32 + 1) * bet;
  }

  total as i32
}

fn part2(input: &str) -> i32 {
  let mut parsed: Vec<(Vec<CardKind>, HandKind, Bet)> = input.lines().map(|line| line.split(' ')).map(|mut split_line| {
    let hand = split_line.next().unwrap();
    let bet = split_line.next().unwrap();
    let parsed_hand: Vec<CardKind> = hand.chars().map(|card| {
      match card {
        '2'..='9' => card.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!()
      }
    }).collect();

    let mut hm = HashMap::new();
    for c in hand.chars() {
      if hm.contains_key(&c) {
        let _ = hm.insert(c, hm.get(&c).unwrap() + 1);
      } else {
        hm.insert(c, 1);
      }
    }

    let j_count = *hm.get(&'J').unwrap_or(&0);
    hm.remove(&'J');
    let hand_kind = match hm.len() {
      5 => 1,
      4 => 2,
      3 => match hm.values().max().unwrap() + j_count {
        2 => 3,
        3 => 4,
        _ => panic!()
      },
      2 => match hm.values().max().unwrap() + j_count {
        3 => 5,
        4 => 6,
        _ => panic!()
      },
      1 => 7,
      0 => 7,
      _ => panic!()
    };
    
    let parsed_bet = bet.parse::<u32>().unwrap();

    (parsed_hand, hand_kind, parsed_bet)
  }).collect();

  parsed.sort_by(|(cards_a, hand_a, _), (cards_b, hand_b, _)| {
    let hand_compare = hand_a.cmp(&hand_b);
    if hand_compare != Ordering::Equal {
      return hand_compare;
    }

    for (card_a, card_b) in cards_a.iter().zip(cards_b.iter()) {
      let card_compare = card_a.cmp(&card_b);
      if card_compare != Ordering::Equal {
        return card_compare;
      }
    }

    panic!();
  });

  let mut total = 0;
  for (i, (_, _, bet)) in parsed.iter().enumerate() {
    total += (i as u32 + 1) * bet;
  }

  total as i32
}

fn main() {
  println!("Test 1: {}", part1(read_input("./test.txt").as_str()));
  println!("Test 2: {}", part2(read_input("./test.txt").as_str()));
  println!("Part 1: {}", part1(read_input("./input.txt").as_str()));
  println!("Part 2: {}", part2(read_input("./input.txt").as_str()));
}
