#[macro_use] extern crate lazy_static;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::read::read_sectioned_input;
use ya_advent_lib::infinite_grid::InfiniteGrid;

#[derive(Copy, Clone)]
enum Fold {
    X(i64),
    Y(i64),
}

impl FromStr for Fold {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([xy])=(\d+)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let v = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            if caps.get(1).unwrap().as_str() == "x" {
                Ok(Fold::X(v))
            } else {
                Ok(Fold::Y(v))
            }
        }
        else {
            Err(())
        }
    }
}

type Input = (Vec<Coord2D>, Vec<Fold>);

fn setup(input: &Input) -> (InfiniteGrid<bool>, Vec<Fold>) {
    let mut grid = InfiniteGrid::new(false);
    let folds = input.1.clone();
    for c in input.0.iter() {
        grid.set(c.x, c.y, true);
    }
    (grid, folds)
}

fn fold(grid: &InfiniteGrid<bool>, fold: &Fold) -> InfiniteGrid<bool> {
    let mut newgrid = InfiniteGrid::new(false);
    for ((x, y), c) in grid.iter() {
        if !c { continue; }
        match fold {
            Fold::X(v) => {
                if *x >= *v {
                    newgrid.set(v - (*x - *v), *y, true);
                } else {
                    newgrid.set(*x, *y, true);
                }
            },
            Fold::Y(v) => {
                if *y >= *v {
                    newgrid.set(*x, v - (*y - *v), true);
                } else {
                    newgrid.set(*x, *y, true);
                }
            },
        }
    }
    newgrid
}

fn part1(input: &Input) -> usize {
    let (grid, folds) = setup(input);
    let grid = fold(&grid, &folds[0]);
    //grid.print(|c| if c { '#' } else { '.' });
    grid.iter().filter(|(_,c)| **c).count()
}

fn part2(input: &Input) {
    let (grid, folds) = setup(input);
    let mut nextgrid = grid;
    for f in folds {
        nextgrid = fold(&nextgrid, &f);
    }
    println!("Part 2:");
    nextgrid.print(|c| if c { '#' } else { '.' });
}

fn main() {
    let input: Input = read_sectioned_input();
    println!("Part 1: {}", part1(&input));
    part2(&input);
}

#[cfg(test)]
mod tests {
    use ya_advent_lib::read::sectioned_test_input;
    use super::*;

    #[test]
    fn day13_test() {
        let input: Input = sectioned_test_input(include_str!("day13.testinput"));
        assert_eq!(part1(&input), 17);
    }
}
