use std::cmp::Reverse;
use std::collections::{HashSet, VecDeque};
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;

fn mkgrid(input: &[String]) -> Grid<i32> {
    Grid::from_input_map(input, 9, 1, |c| match c {
        '0'..='9' => (c as u8 - b'0') as i32,
        _ => panic!(),
    })
}

fn find_low_points(grid: &Grid<i32>) -> Vec<Coord2D> {
    let mut points = Vec::new();
    let xb = grid.x_bounds();
    let yb = grid.y_bounds();
    for y in (yb.start + 1)..(yb.end - 1) {
        for x in (xb.start + 1)..(xb.end - 1) {
            let v = grid.get(x, y);
            if Coord2D::new(x, y)
                .neighbors4()
                .into_iter()
                .all(|n| v < grid.get_c(n)) {
                    points.push(Coord2D::new(x, y));
            }
        }
    }
    points
}

fn find_basin_size(grid: &Grid<i32>, start: Coord2D) -> i32 {
    let mut queue: VecDeque<Coord2D> = VecDeque::new();
    let mut visited: HashSet<Coord2D> = HashSet::new();
    queue.push_back(start);
    let mut count = 0;
    while let Some(c) = queue.pop_front() {
        if grid.get_c(c) < 9 && !visited.contains(&c) {
            count += 1;
            visited.insert(c);
            c.neighbors4().into_iter().for_each(|n| queue.push_back(n));
        }
    }
    count
}

fn part1(grid: &Grid<i32>) -> i32 {
    find_low_points(grid)
        .into_iter()
        .map(|c| grid.get_c(c) + 1)
        .sum()
}

fn part2(grid: &Grid<i32>) -> i32 {
    find_low_points(grid)
        .into_iter()
        .map(|c| find_basin_size(grid, c))
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
