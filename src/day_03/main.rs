use common::read_input;

fn is_symbol(c: char) -> bool {
    return c != '.' && !c.is_numeric();
}

fn borders_symbol(grid: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    let start_x = (j - 1).max(0);
    let start_y = (i - 1).max(0);
    let end_x = (j + 1).min(grid[0].len() as i32 - 1);
    let end_y = (i + 1).min(grid.len() as i32 - 1);

    for y in start_y..=end_y {
        for x in start_x..=end_x {
            if is_symbol(grid[y as usize][x as usize]) {
                return true;
            }
        }
    }

    false
}

fn part1(input: &str) -> i32 {
    let mut total: i32 = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut k = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if k > 0 {
                k -= 1;
                continue;
            }

            if !grid[i][j].is_numeric() {
                continue;
            }

            let mut part = false;
            let mut num: i32 = 0;
            k = 0;
            while (j + k) < grid[i].len() && grid[i][j + k].is_numeric() {
                if part || borders_symbol(&grid, i as i32, (j + k) as i32) {
                    part = true;
                }
                num *= 10;
                num += grid[i][j + k].to_digit(10).unwrap() as i32;
                k += 1;
            }

            k -= 1;

            if part {
                total += num;
            }
        }
    }
    total
}

fn complete_number(grid: &Vec<Vec<char>>, i: i32, j: i32) -> u32 {
    let mut k: i32 = 0;
    let mut off = 1;
    let mut total = 0;
    while j + k >= 0 && grid[i as usize][(j + k) as usize].is_numeric() {
        total += off * grid[i as usize][(j + k) as usize].to_digit(10).unwrap();
        off *= 10;
        k -= 1;
    }
    total
}

fn handle_star(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let start_x = (j as i32 - 1).max(0);
    let start_y = (i as i32 - 1).max(0);
    let end_x = (j as i32 + 1).min(grid[0].len() as i32 - 1);
    let end_y = (i as i32 + 1).min(grid.len() as i32 - 1);

    let mut gears: Vec<u32> = Vec::new();

    let mut k = 0;
    for y in start_y..=end_y {
        for x in start_x..=end_x {
            if k > 0 {
                k -= 1;
                continue;
            }

            if !grid[y as usize][x as usize].is_numeric() {
                continue;
            }

            while ((x + k) as usize) < grid[i].len()
                && grid[y as usize][(x + k) as usize].is_numeric()
            {
                k += 1;
            }
            k -= 1;

            let num = complete_number(&grid, y, x + k);
            gears.push(num);
        }
        k = 0;
    }

    if gears.len() != 2 {
        return 0;
    }

    gears[0] as i32 * gears[1] as i32
}

fn part2(input: &str) -> i32 {
    let mut total: i32 = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != '*' {
                continue;
            }

            total += handle_star(&grid, i, j);
        }
    }

    total
}

fn main() {
    println!("Test 1: {}", part1(read_input("./test.txt").as_str()));
    println!("Test 2: {}", part2(read_input("./test.txt").as_str()));
    println!("Part 1: {}", part1(read_input("./input.txt").as_str()));
    println!("Part 2: {}", part2(read_input("./input.txt").as_str()));
}
