#[macro_use] extern crate lazy_static;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
extern crate advent2021;
use advent2021::read::read_input;
use advent2021::infinite_grid::InfiniteGrid;

enum Input {
    Coord(i64,i64),
    XFold(i64),
    YFold(i64),
    Blank,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref CRE: Regex = Regex::new(r"(\d+),(\d+)").unwrap();
        }
        lazy_static! {
            static ref FRE: Regex = Regex::new(r"([xy])=(\d+)").unwrap();
        }
        if s == "" {
            Ok(Input::Blank)
        } else if let Some(caps) = CRE.captures(s) {
            Ok(Input::Coord(
                caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            ))
        }
        else if let Some(caps) = FRE.captures(s) {
            let v = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            if caps.get(1).unwrap().as_str() == "x" {
                Ok(Input::XFold(v))
            } else {
                Ok(Input::YFold(v))
            }
        }
        else {
            Err(())
        }
    }
}

enum Fold {
    X(i64),
    Y(i64),
}

fn setup(input: &Vec<Input>) -> (InfiniteGrid<bool>, Vec<Fold>) {
    let mut grid = InfiniteGrid::new(false);
    let mut folds = Vec::new();
    for i in input {
        match i {
            Input::Coord(x, y) => {grid.set(*x, *y, true);},
            Input::XFold(v) => {folds.push(Fold::X(*v));},
            Input::YFold(v) => {folds.push(Fold::Y(*v));},
            Input::Blank => {},
        }
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

fn part1(input: &Vec<Input>) -> usize {
    let (grid, folds) = setup(&input);
    let grid = fold(&grid, &folds[0]);
    //grid.print(|c| if c { '#' } else { '.' });
    grid.iter().filter(|(_,c)| **c).count()
}

fn part2(input: &Vec<Input>) {
    let (grid, folds) = setup(&input);
    let mut nextgrid = grid;
    for f in folds {
        nextgrid = fold(&nextgrid, &f);
    }
    println!("Part 2:");
    nextgrid.print(|c| if c { '#' } else { '.' });
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    part2(&input);
}

#[cfg(test)]
mod tests {
    use advent2021::read::test_input;
    use super::*;

    #[test]
    fn day13_test() {
        let input: Vec<Input> = test_input(include_str!("day13.testinput"));
        assert_eq!(part1(&input), 17);
    }
}
