use std::cmp::Reverse;
use std::collections::{HashSet, VecDeque};
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;

fn mkgrid(input: &[String]) -> Grid<i32> {
    Grid::from_input_map(input, 9, 1, |c| match c {
        '0'..='9' => (c as u8 - b'0') as i32,
        _ => panic!(),
    })
}

fn find_low_points(grid: &Grid<i32>) -> Vec<(i64,i64)> {
    let mut points = Vec::new();
    let xb = grid.x_bounds();
    let yb = grid.y_bounds();
    for y in (yb.start + 1)..(yb.end - 1) {
        for x in (xb.start + 1)..(xb.end - 1) {
            let c = grid.get(x, y);
            if c < grid.get(x - 1, y)
                && c < grid.get(x + 1, y)
                && c < grid.get(x, y - 1)
                && c < grid.get(x, y + 1) {
                    points.push((x, y));
            }
        }
    }
    points
}

fn find_basin_size(grid: &Grid<i32>, cx: i64, cy: i64) -> i32 {
    let mut queue: VecDeque<(i64,i64)> = VecDeque::new();
    let mut visited: HashSet<(i64,i64)> = HashSet::new();
    queue.push_back((cx, cy));
    let mut count = 0;
    while let Some((x, y)) = queue.pop_front() {
        if grid.get(x, y) < 9 && !visited.contains(&(x, y)) {
            count += 1;
            visited.insert((x, y));
            queue.push_back((x - 1, y));
            queue.push_back((x + 1, y));
            queue.push_back((x, y - 1));
            queue.push_back((x, y + 1));
        }
    }
    count
}

fn part1(grid: &Grid<i32>) -> i32 {
    find_low_points(grid)
        .into_iter()
        .map(|(x, y)| grid.get(x, y) + 1)
        .sum()
}

fn part2(grid: &Grid<i32>) -> i32 {
    find_low_points(grid)
        .into_iter()
        .map(|(x, y)| find_basin_size(grid, x, y))
        .map(Reverse)
        .k_smallest(3)
        .map(|s| s.0)
        .product()
}

fn main() {
    let input: Vec<String> = read_input();
    let grid = mkgrid(&input);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use ya_advent_lib::read::test_input;
    use super::*;

    #[test]
    fn day09_test() {
        let input: Vec<String> = test_input(include_str!("day09.testinput"));
        let grid = mkgrid(&input);
        assert_eq!(part1(&grid), 15);
        assert_eq!(part2(&grid), 1134);
    }
}
