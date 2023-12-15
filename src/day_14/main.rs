use common::read_input;

type Rock = (usize, usize, bool);

fn roll_north(grid: &mut Vec<Rock>) {
    let mut changed = true;
    while changed {
        changed = false;
        for idx1 in 0..grid.len() {
            let (i, j, movable) = grid[idx1];
            if !movable || i == 0 {
                continue;
            }

            let mut exists = false;

            for idx2 in 0..grid.len() {
                let (i2, j2, _) = grid[idx2];
                if j2 == j && i2 == i - 1 {
                    exists = true;
                    break;
                }
            }

            if !exists {
                grid[idx1].0 -= 1;
                changed = true;
            }
        }
    }
}

fn count_load(grid: &Vec<Rock>, height: usize) -> usize {
    grid.iter()
        .map(|(i, _, movable)| {
            if !movable {
                return 0;
            }

            height - i
        })
        .sum()
}

fn part1(input: &str) -> i32 {
    let raw_grid = input.lines().map(|line| {
        line.chars().map(|c| match c {
            '.' => 0,
            '#' => 1,
            'O' => 2,
            _ => panic!(),
        })
    });
    let height = raw_grid.clone().count();

    let mut rocks: Vec<Rock> = Vec::new();
    for (i, row) in raw_grid.enumerate() {
        for (j, val) in row.enumerate() {
            if val > 0 {
                rocks.push((i, j, val == 2));
            }
        }
    }

    roll_north(&mut rocks);

    count_load(&rocks, height) as i32
}

fn _part2(_input: &str) -> i32 {
    0
}

fn main() {
    println!("Test 1: {}", part1(read_input("./test.txt").as_str()));
    // println!("Test 2: {}", part2(read_input("./test.txt").as_str()));
    println!("Part 1: {}", part1(read_input("./input.txt").as_str()));
    // println!("Part 2: {}", part2(read_input("./input.txt").as_str()));
}
