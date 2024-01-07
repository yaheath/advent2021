use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::infinite_grid::InfiniteGrid;
use ya_advent_lib::range::BidirRangeInclusive;
use ya_advent_lib::read::read_input;

struct Line {
    a: Coord2D,
    b: Coord2D
}

impl FromStr for Line {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" -> ").unwrap();
        Ok(Line {
            a: a.parse::<Coord2D>().unwrap(),
            b: b.parse::<Coord2D>().unwrap(),
        })
    }
}

impl Line {
    fn is_aa(&self) -> bool {
        self.a.y == self.b.y || self.a.x == self.b.x
    }
}

fn doit(input: &[Line], with_diagonals: bool) -> usize {
    let mut grid: InfiniteGrid<u32> = InfiniteGrid::new(0);
    input
        .iter()
        .filter(|l| with_diagonals || l.is_aa())
        .for_each(|l| {
            if l.a.x == l.b.x {
                for y in l.a.y.min(l.b.y)..=l.a.y.max(l.b.y) {
                    let v = grid.get(l.a.x, y);
                    grid.set(l.a.x, y, v+1);
                }
            } else if l.a.y == l.b.y {
                for x in l.a.x.min(l.b.x)..=l.a.x.max(l.b.x) {
                    let v = grid.get(x, l.a.y);
                    grid.set(x, l.a.y, v+1);
                }
            } else {
                BidirRangeInclusive::new(l.a.x, l.b.x).into_iter()
                    .zip(BidirRangeInclusive::new(l.a.y, l.b.y))
                    .for_each(|(x, y)| {
                        let v = grid.get(x, y);
                        grid.set(x, y, v+1);
                    });
            }
        });
    grid.iter().filter(|(_,v)| **v > 1).count()
}

fn part1(input: &[Line]) -> usize {
    doit(input, false)
}

fn part2(input: &[Line]) -> usize {
    doit(input, true)
}

fn main() {
    let input: Vec<Line> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use ya_advent_lib::read::test_input;
    use super::*;

    #[test]
    fn day05_test() {
        let input: Vec<Line> = test_input(include_str!("day05.testinput"));
        assert_eq!(part1(&input), 5);
        assert_eq!(part2(&input), 12);
    }
}
