use std::collections::HashMap;

fn main() {
    let grid = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|n| n.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    star_one(&grid);
    star_two(&grid);
}

fn get_moves(col: usize, row: usize) -> Vec<(usize, usize)> {
    let mut moves = vec![(col + 1, row), (col, row + 1)];

    if col > 0 {
        moves.push((col - 1, row))
    }
    if row > 0 {
        moves.push((col, row - 1))
    }

    moves
}

fn star_one(grid: &Vec<Vec<u32>>) {
    let mut total = 0;
    for col in 0..grid.len() {
        for row in 0..grid[0].len() {
            let current = grid[col][row];
            let moves = get_moves(col, row);
            let mut lowpoint = true;

            for mv in moves {
                let adjacent = match grid.get(mv.0) {
                    Some(c) => match c.get(mv.1) {
                        Some(v) => *v,
                        None => continue,
                    },
                    None => continue,
                };

                if current >= adjacent {
                    lowpoint = false;
                }
            }
            if lowpoint {
                total += 1 + current;
            }
        }
    }

    println!("Total: {}", total);
}

fn r_get_basin_size(
    col: usize,
    row: usize,
    grid: &Vec<Vec<u32>>,
    visited: &mut HashMap<String, bool>,
) -> u32 {
    let key = format!("{}{}", col, row);
    if *visited.get(&key).unwrap_or(&false) {
        return 0;
    };

    visited.insert(key, true);

    let moves = get_moves(col, row);
    let mut size = 1;
    for mv in moves {
        let adjacent = match grid.get(mv.0) {
            Some(c) => match c.get(mv.1) {
                Some(v) => *v,
                None => continue,
            },
            None => continue,
        };
        if adjacent == 9 {
            continue;
        }
        if grid[col][row] < adjacent {
            size += r_get_basin_size(mv.0, mv.1, grid, visited)
        }
    }
    size
}

fn get_basin_size(col: usize, row: usize, grid: &Vec<Vec<u32>>) -> u32 {
    let mut memo = HashMap::new();
    r_get_basin_size(col, row, grid, &mut memo)
}

fn star_two(grid: &Vec<Vec<u32>>) {
    let mut largest_basins = [0, 0, 0];
    for col in 0..grid.len() {
        for row in 0..grid[0].len() {
            let current = grid[col][row];
            let moves = get_moves(col, row);
            let mut lowpoint = true;

            for mv in moves {
                let adjacent = match grid.get(mv.0) {
                    Some(c) => match c.get(mv.1) {
                        Some(v) => *v,
                        None => continue,
                    },
                    None => continue,
                };

                if current >= adjacent {
                    lowpoint = false;
                }
            }
            if lowpoint {
                let basin_size = get_basin_size(col, row, grid);
                let min = largest_basins.iter_mut().min();
                if let Some(v) = min {
                    if *v < basin_size {
                        *v = basin_size
                    }
                }
            }
        }
    }

    println!("{:?}", largest_basins);
    let total: u32 = largest_basins.iter().product();
    println!("total: {}", total);
}
