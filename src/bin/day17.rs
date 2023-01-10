#[macro_use] extern crate lazy_static;
use std::ops::Range;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
extern crate advent2021;
use advent2021::read::read_input;

struct TargetArea {
    x: Range<i64>,
    y: Range<i64>,
}

impl FromStr for TargetArea {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let x1 = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let x2 = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let y1 = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let y2 = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
            Ok(TargetArea {
                x: Range {start: x1.min(x2), end: x1.max(x2) + 1},
                y: Range {start: y1.min(y2), end: y1.max(y2) + 1},
            })
        }
        else {
            Err(())
        }
    }
}

enum ShotResult {
    Undershot,
    Overshot,
    Missed,
    Hit(i64),
}

fn sim_shot(x_vel: i64, y_vel: i64, target: &TargetArea) -> ShotResult {
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut x_vel = x_vel;
    let mut y_vel = y_vel;
    let mut max_y = 0;
    loop {
        if target.x.contains(&x_pos) && target.y.contains(&y_pos) {
            return ShotResult::Hit(max_y);
        }
        if y_pos < target.y.start && x_pos < target.x.end {
            return ShotResult::Undershot;
        }
        if x_pos >= target.x.end && y_pos <= target.y.start {
            return ShotResult::Overshot;
        }
        if x_pos >= target.x.end && y_pos < target.y.start {
            return ShotResult::Missed;
        }
        x_pos += x_vel;
        y_pos += y_vel;
        x_vel += if x_vel < 0 { 1 } else if x_vel > 0 { -1 } else { 0 };
        y_vel -= 1;
        max_y = max_y.max(y_pos);
    }
}

fn find_target(target: &TargetArea) -> (i64, i64, Option<i64>) {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = None;
    loop {
        match sim_shot(x, y, target) {
            ShotResult::Undershot => { x += 1; y += 1; },
            ShotResult::Hit(hgt) => { max_y = Some(hgt); break; },
            _ => { break; },
        }
    }
    (x, y, max_y)
}

fn find_all_hits(target: &TargetArea) -> (i64, usize) {
    let (cx, cy, max_y) = find_target(target);
    let mut r = 1;
    let mut needhit = max_y.is_none();
    let mut max_y = max_y.unwrap_or(0);
    let mut n_hits = if needhit { 0 } else { 1 };
    let mut direct_hit = false;
    loop {
        let mut nohits = true;
        ((cx-r)..=(cx+r)).map(|x| (x, cy-r))
            .chain(
                ((cx-r)..=(cx+r)).map(|x| (x, cy+r))
            ).chain(
                ((cy-r+1)..=(cy+r-1)).map(|y| (cx-r, y))
            ).chain(
                ((cy-r+1)..=(cy+r-1)).map(|y| (cx+r, y))
            ).for_each(|(x, y)| {
                if target.x.contains(&x) && target.y.contains(&y) {
                    direct_hit = true;
                }
                match sim_shot(x, y, target) {
                    ShotResult::Hit(hgt) => {
                        needhit = false;
                        nohits = false;
                        max_y = max_y.max(hgt);
                        n_hits += 1;
                    },
                    _ => {},
                }
            });
        if nohits && !needhit && direct_hit {
            break;
        }
        r += 1;
    }
    (max_y, n_hits)
}

fn main() {
    let input: Vec<TargetArea> = read_input();
    let (max_y, n_hits) = find_all_hits(&input[0]);
    println!("Part 1: {max_y}");
    println!("Part 2: {n_hits}");
}

#[cfg(test)]
mod tests {
    use advent2021::read::test_input;
    use super::*;

    #[test]
    fn day17_test() {
        let input: Vec<TargetArea> = test_input("target area: x=20..30, y=-10..-5");
        let (max_y, n_hits) = find_all_hits(&input[0]);
        assert_eq!(max_y, 45);
        assert_eq!(n_hits, 112);
    }
}
