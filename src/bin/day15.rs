use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::vec::Vec;
use advent_lib::grid::Grid;
use advent_lib::read::read_input;

fn mkgrid(input: &Vec<String>) -> Grid<u8> {
    Grid::from_input(input, 0, 0, |c| match c {
        '0'..='9' => (c as u8 - b'0') as u8,
        _ => panic!(),
    })
}

fn up(loc: (i64, i64)) -> (i64, i64) { (loc.0, loc.1 - 1) }
fn down(loc: (i64, i64)) -> (i64, i64) { (loc.0, loc.1 + 1) }
fn left(loc: (i64, i64)) -> (i64, i64) { (loc.0 - 1, loc.1) }
fn right(loc: (i64, i64)) -> (i64, i64) { (loc.0 + 1, loc.1) }

fn traverse<F>(getter: F, width: i64, height: i64) -> usize
        where F: Fn((i64, i64)) -> u8 {
    let target = ( width - 1, height - 1 );
    let h = |(x,y)| ((target.0 - x) + (target.1 - y)) as usize;

    let mut queue: BinaryHeap<(Reverse<usize>,(i64,i64))> = BinaryHeap::new();
    queue.push((Reverse(h((0, 0))), (0, 0)));
    let mut traversed: HashMap<(i64,i64),usize> = HashMap::new();
    traversed.insert((0, 0), 0);

    while let Some((_, loc)) = queue.pop() {
        if loc == target {
            return traversed[&loc];
        }
        let cur_risk = traversed[&(loc.0, loc.1)];
        for p in [up(loc), down(loc), left(loc), right(loc)] {
            if p.0 >= 0 && p.0 < width
                && p.1 >= 0 && p.1 < height
            {
                let d = getter(p) as usize + cur_risk;
                if !traversed.contains_key(&p) || d < traversed[&p] {
                    traversed.insert(p, d);
                    queue.push((Reverse(d + h(p)), p));
                }
            }
        }
    }
    panic!();
}

fn part1(grid: &Grid<u8>) -> usize {
    traverse(|(x, y)| grid.get(x, y), grid.x_bounds().end, grid.y_bounds().end)
}

fn part2(grid: &Grid<u8>) -> usize {
    let width = grid.x_bounds().end;
    let height = grid.y_bounds().end;
    traverse(|(x, y)| {
        let v = grid.get(x % width, y % width) - 1;
        let xs = (x / width) as u8;
        let ys = (y / width) as u8;
        ((v + xs + ys) % 9) + 1
    }, width * 5, height * 5)
}

fn main() {
    let input: Vec<String> = read_input();
    let grid = mkgrid(&input);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use advent_lib::read::test_input;
    use super::*;

    #[test]
    fn day15_test() {
        let input: Vec<String> = test_input(include_str!("day15.testinput"));
        let grid = mkgrid(&input);
        assert_eq!(part1(&grid), 40);
        assert_eq!(part2(&grid), 315);
    }
}
