use petgraph::graph::Graph;
use std::collections::HashMap;

type Point = (usize, usize);

fn main() {
    let mut g = Graph::new_undirected();

    let grid = include_str!("../test.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| g.add_node(c.to_digit(10).unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", grid);

    let mut memo = HashMap::new();

    get_path((0, 0), &grid, &mut memo)
}

fn get_path(point: Point, grid: &[Vec<u32>], memo: &mut HashMap<Point, u32>) {
    let entry = memo.entry(point).or_insert(grid[point.0][point.1]);

    if memo.contains_key(&point) {}
}
