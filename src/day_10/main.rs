use common::read_input;

enum Dir {
    Up,
    Right,
    Down,
    Left,
}

use Dir::{Down, Left, Right, Up};

fn find_start(v: &Vec<Vec<char>>) -> (usize, usize, Dir) {
    for i in 0..v.len() {
        for j in 0..v[0].len() {
            if v[i][j] == 'S' {
                if i > 0 && "7|F".contains(v[i - 1][j]) {
                    return (j, i, Up);
                }
                if j > 0 && "J-7".contains(v[i][j - 1]) {
                    return (j, i, Left);
                }
                if i < v.len() - 1 && "L|J".contains(v[i + 1][j]) {
                    return (j, i, Down);
                }
                if j < v[0].len() - 1 && "L-F".contains(v[i][j + 1]) {
                    return (j, i, Right);
                }
            }
        }
    }
    panic!();
}

fn connect(c: char, dir: Dir) -> Option<Dir> {
    match dir {
        Up => match c {
            '7' => Some(Left),
            '|' => Some(Up),
            'F' => Some(Right),
            'S' => Some(Up),
            _ => None,
        },
        Right => match c {
            'J' => Some(Up),
            '-' => Some(Right),
            '7' => Some(Down),
            'S' => Some(Up),
            _ => None,
        },
        Down => match c {
            'L' => Some(Right),
            '|' => Some(Down),
            'J' => Some(Left),
            'S' => Some(Up),
            _ => None,
        },
        Left => match c {
            'F' => Some(Down),
            '-' => Some(Left),
            'L' => Some(Up),
            'S' => Some(Up),
            _ => None,
        },
    }
}

fn part1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (s_x, s_y, s_dir) = find_start(&grid);
    let mut x = s_x;
    let mut y = s_y;
    let mut dir = s_dir;
    let mut steps = 0;
    loop {
        (x, y, dir) = match dir {
            Up => (x, y - 1, connect(grid[y - 1][x], Up).unwrap()),
            Right => (x + 1, y, connect(grid[y][x + 1], Right).unwrap()),
            Down => (x, y + 1, connect(grid[y + 1][x], Down).unwrap()),
            Left => (x - 1, y, connect(grid[y][x - 1], Left).unwrap()),
        };

        steps += 1;

        if s_x == x && s_y == y {
            break;
        }
    }

    steps / 2
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
