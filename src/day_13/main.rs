use common::read_input;

fn find_vertical_mirror(grid: &Vec<Vec<bool>>, err: usize) -> Option<usize> {
    let ysize = grid.len();
    let xsize = grid[0].len();
    for i in 1..ysize {
        let checksize = i.min(ysize - i);
        let mut errors = 0;
        for s in 1..=checksize {
            for j in 0..xsize {
                if grid[i - s][j] != grid[i + s - 1][j] {
                    errors += 1;
                }
            }
        }

        if errors == err {
            return Some(i);
        }
    }
    None
}

fn find_horizontal_mirror(grid: &Vec<Vec<bool>>, err: usize) -> Option<usize> {
    let xsize = grid[0].len();
    let mut trans_grid: Vec<Vec<bool>> = Vec::new();
    for i in 0..xsize {
        trans_grid.push(grid.iter().map(|row| row[i]).collect());
    }
    find_vertical_mirror(&trans_grid, err)
}

fn part1(input: &str) -> i32 {
    let grids: Vec<Vec<Vec<bool>>> = input
        .split("\n\n")
        .map(|grid| {
            grid.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => false,
                            '#' => true,
                            _ => panic!(),
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut total: usize = 0;
    for grid in grids {
        if let Some(pos) = find_vertical_mirror(&grid, 0) {
            total += 100 * pos;
        } else if let Some(pos) = find_horizontal_mirror(&grid, 0) {
            total += pos;
        } else {
            println!("None");
        }
    }

    total as i32
}

fn part2(input: &str) -> i32 {
    let grids: Vec<Vec<Vec<bool>>> = input
        .split("\n\n")
        .map(|grid| {
            grid.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => false,
                            '#' => true,
                            _ => panic!(),
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut total: usize = 0;
    for grid in grids {
        if let Some(pos) = find_vertical_mirror(&grid, 1) {
            total += 100 * pos;
        } else if let Some(pos) = find_horizontal_mirror(&grid, 1) {
            total += pos;
        } else {
            println!("None");
        }
    }

    total as i32
}

fn main() {
    println!("Test 1: {}", part1(read_input("./test.txt").as_str()));
    println!("Test 2: {}", part2(read_input("./test.txt").as_str()));
    println!("Part 1: {}", part1(read_input("./input.txt").as_str()));
    println!("Part 2: {}", part2(read_input("./input.txt").as_str()));
}
