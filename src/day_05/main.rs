use common::read_input;
fn read_seeds(lines: Vec<&str>) -> Vec<isize> {
    lines
        .iter()
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|num| num.parse::<isize>().unwrap())
        .collect()
}

fn read_ranges(lines: Vec<&str>) -> Vec<Vec<isize>> {
    lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect()
}

fn apply_ranges(seeds: Vec<isize>, ranges: Vec<Vec<isize>>) -> Vec<isize> {
    seeds
        .iter()
        .cloned()
        .map(|seed| {
            for range in ranges.iter() {
                if seed >= range[1] && seed < range[1] + range[2] {
                    return range[0] - range[1] + seed;
                }
            }
            return seed;
        })
        .collect()
}

fn part1(input: &str) -> isize {
    let mut lines = input.lines();
    let mut seeds = read_seeds(lines.clone().collect::<Vec<&str>>());

    for _ in 0..3 {
        let _ = lines.next();
    }

    let mut ranges = read_ranges(lines.clone().collect::<Vec<&str>>());
    seeds = apply_ranges(seeds, ranges.clone());

    for _ in 0..6 {
        for _ in 0..ranges.len() + 2 {
            let _ = lines.next();
        }

        ranges = read_ranges(lines.clone().collect::<Vec<&str>>());
        seeds = apply_ranges(seeds, ranges.clone());
    }

    seeds
        .iter()
        .fold(isize::MAX, |mini: isize, x: &isize| mini.min(*x))
}

fn part2(input: &str) -> isize {
    let mut groups = input.split("\n\n");
    let mut numbers = groups
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|num| num.parse::<isize>().unwrap());

    let mut seeds = Vec::new();
    while let Some(num) = numbers.next() {
        seeds.push((num, num + numbers.next().unwrap()));
    }

    let mut new_seeds = Vec::new();
    let mut tbd = Vec::new();

    for group in groups {
        new_seeds.clear();

        for line in group.lines().skip(1) {
            tbd.clear();

            let mut numbers = line.split(' ').map(|num| num.parse::<isize>().unwrap());
            let new_range_start = numbers.next().unwrap();
            let range_start = numbers.next().unwrap();
            let range_size = numbers.next().unwrap();
            let range_end = range_start + range_size;

            for &(seed_start, seed_end) in &seeds {
                let maybe_start = seed_start.max(range_end);
                let maybe_end = seed_end.min(range_start);
                if seed_start < maybe_end {
                    tbd.push((seed_start, maybe_end));
                }
                if maybe_start < seed_end {
                    tbd.push((maybe_start, seed_end));
                }

                let new_start =
                    seed_start.max(range_start).min(range_end) + (new_range_start - range_start);
                let new_end =
                    seed_end.min(range_end).max(range_start) + (new_range_start - range_start);
                if new_end > new_start {
                    new_seeds.push((new_start, new_end));
                }
            }

            std::mem::swap(&mut seeds, &mut tbd);
        }

        new_seeds.extend(seeds.iter().copied());
        std::mem::swap(&mut seeds, &mut new_seeds);
    }

    seeds.iter().min().unwrap().0
}

fn main() {
    println!("Test 1: {}", part1(read_input("./test.txt").as_str()));
    println!("Test 2: {}", part2(read_input("./test.txt").as_str()));
    println!("Part 1: {}", part1(read_input("./input.txt").as_str()));
    println!("Part 2: {}", part2(read_input("./input.txt").as_str()));
}
