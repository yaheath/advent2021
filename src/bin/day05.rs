#[macro_use] extern crate lazy_static;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
extern crate advent2021;
use advent2021::read::read_input;
use advent2021::infinite_grid::InfiniteGrid;
use advent2021::range::BidirRangeInclusive;

struct Line {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

impl FromStr for Line {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+),(\d+) .. (\d+),(\d+)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            Ok(Line {
                x1: caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                y1: caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                x2: caps.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                y2: caps.get(4).unwrap().as_str().parse::<i64>().unwrap(),
            })
        }
        else {
            Err(())
        }
    }
}

impl Line {
    fn is_aa(&self) -> bool {
        self.y1 == self.y2 || self.x1 == self.x2
    }
}

fn doit(input: &Vec<Line>, with_diagonals: bool) -> usize {
    let mut grid: InfiniteGrid<u32> = InfiniteGrid::new(0);
    input
        .iter()
        .filter(|l| with_diagonals || l.is_aa())
        .for_each(|l| {
            if l.x1 == l.x2 {
                for y in l.y1.min(l.y2)..=l.y1.max(l.y2) {
                    let v = grid.get(l.x1, y);
                    grid.set(l.x1, y, v+1);
                }
            } else if l.y1 == l.y2 {
                for x in l.x1.min(l.x2)..=l.x1.max(l.x2) {
                    let v = grid.get(x, l.y1);
                    grid.set(x, l.y1, v+1);
                }
            } else {
                BidirRangeInclusive::new(l.x1, l.x2).into_iter()
                    .zip(BidirRangeInclusive::new(l.y1, l.y2).into_iter())
                    .for_each(|(x, y)| {
                        let v = grid.get(x, y);
                        grid.set(x, y, v+1);
                    });
            }
        });
    grid.iter().filter(|(_,v)| **v > 1).count()
}

fn part1(input: &Vec<Line>) -> usize {
    doit(input, false)
}

fn part2(input: &Vec<Line>) -> usize {
    doit(input, true)
}

fn main() {
    let input: Vec<Line> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use advent2021::read::test_input;
    use super::*;

    #[test]
    fn day05_test() {
        let input: Vec<Line> = test_input(include_str!("day05.testinput"));
        assert_eq!(part1(&input), 5);
        assert_eq!(part2(&input), 12);
    }
}
