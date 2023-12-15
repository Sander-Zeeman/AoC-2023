use common::read_input;

fn hash(s: &str) -> usize {
    let bs = s.as_bytes();
    let mut curr = 0;
    for b in bs {
        curr += *b as usize;
        curr *= 17;
        curr %= 256;
    }
    curr
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|code| hash(code))
        .sum::<usize>() as i32
}

fn _part2(_input: &str) -> i32 {
    return 0;
}

fn main() {
    println!("Test 1: {}", part1(read_input("./test.txt").as_str()));
    // println!("Test 2: {}", part2(read_input("./test.txt").as_str()));
    println!("Part 1: {}", part1(read_input("./input.txt").as_str()));
    // println!("Part 2: {}", part2(read_input("./input.txt").as_str()));
}
