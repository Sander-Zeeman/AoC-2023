use common::read_input;

fn matches(record: &[u8], code: &Vec<i32>) -> bool {
    let mut groups: Vec<i32> = Vec::new();
    let size = record.len();
    let mut i = 0;

    while i < size {
        while i < size && record[i] == b'.' {
            i += 1;
        }

        let mut group_size = 0;

        while i < size && record[i] == b'#' {
            i += 1;
            group_size += 1;
        }

        if group_size > 0 {
            groups.push(group_size);
        }
    }

    *code == groups
}

fn solve(record: &mut String, code: &Vec<i32>) -> i32 {
    let mut total: i32 = 0;

    let count = record.matches("?").count() as u32;

    for i in 0..2_i32.pow(count) {
        let mut copy = record.clone();

        let mut n = i;
        for _ in 0..count {
            let replacement = match n % 2 == 0 {
                true => ".",
                false => "#",
            };
            n /= 2;
            copy = copy.replacen("?", replacement, 1);
        }

        total += match matches(copy.as_bytes(), code) {
            true => 1,
            false => 0,
        };
    }

    total
}

fn part1(input: &str) -> i32 {
    let mut total: i32 = 0;
    for (i, line) in input.lines().enumerate() {
        println!("Starting {}", i);
        let mut split = line.split(' ');
        let mut record = split.next().unwrap().to_string();
        let code: Vec<i32> = split
            .next()
            .unwrap()
            .split(',')
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        total += solve(&mut record, &code);
    }

    total
}

fn part2(input: &str) -> i32 {
    let mut total: i32 = 0;
    for (i, line) in input.lines().enumerate() {
        println!("Starting {}", i);
        let mut split = line.split(' ');
        let mut record = split.next().unwrap().to_string();
        let mut code: Vec<i32> = split
            .next()
            .unwrap()
            .split(',')
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        let record_c = record.clone();
        let code_c = code.clone();
        for _ in 0..5 {
            record.push('?');
            for c in record_c.chars() {
                record.push(c);
            }
            for c in &code_c {
                code.push(*c);
            }
        }
        total += solve(&mut record, &code);
    }

    total
}

fn main() {
    // println!("Test 1: {}", part1(read_input("./test.txt").as_str()));
    println!("Test 2: {}", part2(read_input("./test.txt").as_str()));
    // println!("Part 1: {}", part1(read_input("./input.txt").as_str()));
    // println!("Part 2: {}", part2(read_input("./input.txt").as_str()));
}
