pub fn part1(input: String) {
    let grid = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut is_visible = vec![vec![false; width]; height];

    let mut memo = grid[0].clone();
    for i in 1..(height - 1) {
        #[allow(clippy::needless_range_loop)]
        for j in 1..(width - 1) {
            if grid[i][j] > memo[j] {
                is_visible[i][j] = true;
                memo[j] = grid[i][j];
            }
        }
    }

    let mut memo = grid[height - 1].clone();
    for i in (1..(height - 1)).rev() {
        #[allow(clippy::needless_range_loop)]
        for j in 1..(width - 1) {
            if grid[i][j] > memo[j] {
                is_visible[i][j] = true;
                memo[j] = grid[i][j];
            }
        }
    }

    let mut memo = grid.iter().map(|row| row[0]).collect::<Vec<_>>();
    for j in 1..(width - 1) {
        #[allow(clippy::needless_range_loop)]
        for i in 1..(height - 1) {
            if grid[i][j] > memo[i] {
                is_visible[i][j] = true;
                memo[i] = grid[i][j];
            }
        }
    }

    let mut memo = grid.iter().map(|row| row[width - 1]).collect::<Vec<_>>();
    for j in (1..(width - 1)).rev() {
        #[allow(clippy::needless_range_loop)]
        for i in 1..(height - 1) {
            if grid[i][j] > memo[i] {
                is_visible[i][j] = true;
                memo[i] = grid[i][j];
            }
        }
    }

    let ans = is_visible.iter().flatten().filter(|&&x| x).count() + height * 2 + width * 2 - 4;
    println!("{}", ans);
}

pub fn part2(input: String) {
    let grid = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut score_north: Vec<Vec<usize>> = vec![vec![0; width]; height];
    let mut score_south: Vec<Vec<usize>> = vec![vec![0; width]; height];
    let mut score_west: Vec<Vec<usize>> = vec![vec![0; width]; height];
    let mut score_east: Vec<Vec<usize>> = vec![vec![0; width]; height];

    for i in 1..height {
        for j in 0..width {
            score_north[i][j] = 1;
            if grid[i][j] > grid[i - 1][j] {
                score_north[i][j] += score_north[i - 1][j];
                let mut row = i - 1 - score_north[i - 1][j];
                while score_north[row][j] > 0 {
                    if grid[i][j] > grid[row][j] {
                        score_north[i][j] += score_north[row][j];
                        row -= score_north[row][j];
                    } else {
                        break;
                    }
                }
            }
        }
    }

    for i in (0..(height - 1)).rev() {
        for j in 0..width {
            score_south[i][j] = 1;
            if grid[i][j] > grid[i + 1][j] {
                score_south[i][j] += score_south[i + 1][j];
                let mut row = i + 1 + score_south[i + 1][j];
                while score_south[row][j] > 0 {
                    if grid[i][j] > grid[row][j] {
                        score_south[i][j] += score_south[row][j];
                        row += score_south[row][j];
                    } else {
                        break;
                    }
                }
            }
        }
    }

    for j in 1..width {
        for i in 0..height {
            score_west[i][j] = 1;
            if grid[i][j] > grid[i][j - 1] {
                score_west[i][j] += score_west[i][j - 1];
                let mut col = j - 1 - score_west[i][j - 1];
                while score_west[i][col] > 0 {
                    if grid[i][j] > grid[i][col] {
                        score_west[i][j] += score_west[i][col];
                        col -= score_west[i][col];
                    } else {
                        break;
                    }
                }
            }
        }
    }

    for j in (0..(width - 1)).rev() {
        for i in 0..height {
            score_east[i][j] = 1;
            if grid[i][j] > grid[i][j + 1] {
                score_east[i][j] += score_east[i][j + 1];
                let mut col = j + 1 + score_east[i][j + 1];
                while score_east[i][col] > 0 {
                    if grid[i][j] > grid[i][col] {
                        score_east[i][j] += score_east[i][col];
                        col += score_east[i][col];
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let max_score = score_north
        .iter()
        .flatten()
        .zip(score_south.iter().flatten())
        .zip(score_west.iter().flatten())
        .zip(score_east.iter().flatten())
        .map(|(((a, b), c), d)| a * b * c * d)
        .max()
        .unwrap();

    println!("{}", max_score);
}
