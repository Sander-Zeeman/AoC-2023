use common::read_input;

fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!original.is_empty());
    let mut transposed = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in original {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }

    transposed
}

fn part1(input: &str) -> i32 {
    let universe: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut galaxies: Vec<(i32, i32)> = Vec::new();
    for (i, row) in universe.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.push((i as i32, j as i32));
            }
        }
    }

    let mut x_empty = Vec::new();
    let mut y_empty = Vec::new();

    for (i, row) in universe.iter().enumerate() {
        let size = row.iter().filter(|c| **c == '#').count();
        if size == 0 {
            y_empty.push(i);
        }
    }

    for (i, row) in transpose::<char>(universe).iter().enumerate() {
        let size = row.iter().filter(|c| **c == '#').count();
        if size == 0 {
            x_empty.push(i);
        }
    }

    for galaxy in &mut galaxies {
        let mut x_total = 0;
        let mut y_total = 0;

        for x in x_empty.iter() {
            let x = *x as i32;
            if galaxy.1 > x {
                x_total += 1;
            }
        }
        for y in y_empty.iter() {
            let y = *y as i32;
            if galaxy.0 > y {
                y_total += 1;
            }
        }

        galaxy.0 += y_total;
        galaxy.1 += x_total;
    }

    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let g1 = galaxies[i];
            let g2 = galaxies[j];
            let dist = (g1.0 - g2.0).abs() + (g1.1 - g2.1).abs();
            total += dist;
        }
    }

    total
}

fn part2(input: &str) -> isize {
    let universe: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut galaxies: Vec<(isize, isize)> = Vec::new();
    for (i, row) in universe.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.push((i as isize, j as isize));
            }
        }
    }

    let mut x_empty = Vec::new();
    let mut y_empty = Vec::new();

    for (i, row) in universe.iter().enumerate() {
        let size = row.iter().filter(|c| **c == '#').count();
        if size == 0 {
            y_empty.push(i);
        }
    }

    for (i, row) in transpose::<char>(universe).iter().enumerate() {
        let size = row.iter().filter(|c| **c == '#').count();
        if size == 0 {
            x_empty.push(i);
        }
    }

    for galaxy in &mut galaxies {
        let mut x_total = 0;
        let mut y_total = 0;

        for x in x_empty.iter() {
            let x = *x as isize;
            if galaxy.1 > x {
                x_total += 999999;
            }
        }
        for y in y_empty.iter() {
            let y = *y as isize;
            if galaxy.0 > y {
                y_total += 999999;
            }
        }

        galaxy.0 += y_total;
        galaxy.1 += x_total;
    }

    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let g1 = galaxies[i];
            let g2 = galaxies[j];
            let dist = (g1.0 - g2.0).abs() + (g1.1 - g2.1).abs();
            total += dist;
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
