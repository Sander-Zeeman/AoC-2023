use common::read_input;

fn part1(input: &str) -> i32 {
    let parsed: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut total: i32 = 1;
    for (time, dist) in parsed[0].iter().zip(parsed[1].iter()) {
        let time = *time as f32;
        let dist = *dist as f32 + 1.0;
        // f:      x*total - x*x
        let low = (-0.5 * (-1_f32 * time + (time * time - 4_f32 * dist).sqrt())).ceil();
        let high = 1.0 + (-0.5 * (-1_f32 * time - (time * time - 4_f32 * dist).sqrt())).floor();
        total *= high as i32 - low as i32;
    }

    total
}

fn part2(input: &str) -> i32 {
    let parsed: Vec<usize> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .join("")
                .parse::<usize>()
                .unwrap()
        })
        .collect();

    let time = parsed[0] as f64;
    let dist = parsed[1] as f64 + 1.0;
    let low = (-0.5 * (-1.0 * time + (time * time - 4.0 * dist).sqrt())).ceil();
    let high = 1.0 + (-0.5 * (-1.0 * time - (time * time - 4.0 * dist).sqrt())).floor();
    high as i32 - low as i32
}

fn main() {
    println!("Test 1: {}", part1(read_input("./test.txt").as_str()));
    println!("Test 2: {}", part2(read_input("./test.txt").as_str()));
    println!("Part 1: {}", part1(read_input("./input.txt").as_str()));
    println!("Part 2: {}", part2(read_input("./input.txt").as_str()));
}
