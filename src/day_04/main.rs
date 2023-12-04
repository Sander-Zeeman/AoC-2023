use common::read_input;

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<Vec<i32>> = line
                .split(':')
                .last()
                .unwrap()
                .split('|')
                .map(|num_list| {
                    num_list
                        .split_whitespace()
                        .filter(|num| num.len() > 0)
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect()
                })
                .collect();
            let matches: Vec<&i32> = numbers[0]
                .iter()
                .filter(|number| numbers[1].contains(&number))
                .collect();
            if matches.len() == 0 {
                return 0;
            }

            2_i32.pow(matches.len() as u32 - 1)
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let games: Vec<Vec<Vec<i32>>> = input
        .lines()
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .split('|')
                .map(|num_list| {
                    num_list
                        .split_whitespace()
                        .filter(|num| num.len() > 0)
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    let matches: Vec<usize> = games
        .iter()
        .map(|game| {
            game[0]
                .iter()
                .filter(|number| game[1].contains(&number))
                .collect::<Vec<_>>()
                .len()
        })
        .collect();

    let mut copies = vec![1; games.len()];
    for (i, _) in &mut games.iter().enumerate() {
        for j in 1..=matches[i] {
            copies[i + j] += copies[i];
        }
    }
    copies.iter().sum::<i32>()
}

fn main() {
    println!("Test 1: {}", part1(read_input("./test.txt").as_str()));
    println!("Test 2: {}", part2(read_input("./test.txt").as_str()));
    println!("Part 1: {}", part1(read_input("./input.txt").as_str()));
    println!("Part 2: {}", part2(read_input("./input.txt").as_str()));
}
