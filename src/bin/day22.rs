use std::ops::RangeInclusive;
use std::str::FromStr;
use std::vec::Vec;
extern crate advent2021;
use advent2021::read::read_input;

#[derive(Clone)]
struct Region {
    on: bool,
    rect: Rect3D,
}

type Volume = i64;

impl FromStr for Region {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let on = split.next().unwrap() == "on";
        let split = split.next().unwrap().split(',');
        let vals: Vec<Vec<i64>> = split
            .map(|s| s.split_at(2).1)
            .map(|s| s.split("..").map(|ss| ss.parse::<i64>().unwrap()).collect::<Vec<i64>>())
            .collect();

        Ok(Region{
            on,
            rect: Rect3D {
                x_range: RangeInclusive::new(vals[0][0], vals[0][1]),
                y_range: RangeInclusive::new(vals[1][0], vals[1][1]),
                z_range: RangeInclusive::new(vals[2][0], vals[2][1]),
            },
        })
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Rect3D {
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
    z_range: RangeInclusive<i64>,
}

trait Intersection: Sized {
    fn intersection(&self, other: &Self) -> Option<Self>;
}

impl Intersection for RangeInclusive<i64> {
    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.contains(other.start()) {
            Some(*other.start() ..= (*other.end()).min(*self.end()))
        } else if self.contains(other.end()) {
            Some(*self.start() ..= *other.end())
        } else if other.contains(self.start()) {
            Some(*self.start() ..= *self.end())
        } else {
            None
        }
    }
}

impl Rect3D {
    fn volume(&self) -> Volume {
        (self.x_range.end() - self.x_range.start() + 1) *
        (self.y_range.end() - self.y_range.start() + 1) *
        (self.z_range.end() - self.z_range.start() + 1)
    }
}

impl Intersection for Rect3D {
    fn intersection(&self, other: &Rect3D) -> Option<Rect3D> {
        let x_range = self.x_range.intersection(&other.x_range)?;
        let y_range = self.y_range.intersection(&other.y_range)?;
        let z_range = self.z_range.intersection(&other.z_range)?;
        Some(Rect3D {
            x_range,
            y_range,
            z_range,
        })
    }
}

impl Region {
    fn split(&self, other: &Region) -> Vec<Region> {
        let mut ret = Vec::new();
        let common = self.rect.intersection(&other.rect);
        if common.is_none() {
            ret.push(self.clone());
            return ret;
        }
        let common = common.unwrap();
        if common == self.rect {
            // other completely encapsulates self
            // so we return an empty vec
            return ret;
        }
        if *common.z_range.start() > *self.rect.z_range.start() {
            // above slice
            ret.push(Region {
                on: self.on,
                rect: Rect3D {
                    x_range: self.rect.x_range.clone(),
                    y_range: self.rect.y_range.clone(),
                    z_range: *self.rect.z_range.start() ..= *common.z_range.start() - 1,
                }
            });
        }
        if *common.z_range.end() < *self.rect.z_range.end() {
            // below slice
            ret.push(Region {
                on: self.on,
                rect: Rect3D {
                    x_range: self.rect.x_range.clone(),
                    y_range: self.rect.y_range.clone(),
                    z_range: *common.z_range.end() + 1 ..= *self.rect.z_range.end(),
                }
            });
        }
        if *common.y_range.start() > *self.rect.y_range.start() {
            ret.push(Region {
                on: self.on,
                rect: Rect3D {
                    x_range: self.rect.x_range.clone(),
                    y_range: *self.rect.y_range.start() ..= *common.y_range.start() - 1,
                    z_range: common.z_range.clone(),
                }
            });
        }
        if *common.y_range.end() < *self.rect.y_range.end() {
            ret.push(Region {
                on: self.on,
                rect: Rect3D {
                    x_range: self.rect.x_range.clone(),
                    y_range: *common.y_range.end() + 1 ..= *self.rect.y_range.end(),
                    z_range: common.z_range.clone(),
                }
            });
        }
        if *common.x_range.start() > *self.rect.x_range.start() {
            ret.push(Region {
                on: self.on,
                rect: Rect3D {
                    x_range: *self.rect.x_range.start() ..= *common.x_range.start() - 1,
                    y_range: common.y_range.clone(),
                    z_range: common.z_range.clone(),
                }
            });
        }
        if *common.x_range.end() < *self.rect.x_range.end() {
            ret.push(Region {
                on: self.on,
                rect: Rect3D {
                    x_range: *common.x_range.end() + 1 ..= *self.rect.x_range.end(),
                    y_range: common.y_range.clone(),
                    z_range: common.z_range.clone(),
                }
            });
        }
        ret
    }
    fn intersection(&self, other: &Rect3D) -> Option<Region> {
        if let Some(common) = self.rect.intersection(other) {
            Some(Region { on: self.on, rect: common })
        }
        else {
            None
        }
    }
}


fn add_region(space: Vec<Region>, new: Region) -> Vec<Region> {
    let mut ret:Vec<Region> =
        space.iter().map(|s| s.split(&new)).flatten().collect();
    ret.push(new);
    ret
}

fn build_space(regions: &mut dyn Iterator<Item=Region>) -> Vec<Region> {
    let mut space = Vec::new();
    for r in regions {
        space = add_region(space, r);
    }
    space
}

fn part1(input: &Vec<Region>) -> Volume {
    let bounds = Rect3D {
        x_range: -50..=50,
        y_range: -50..=50,
        z_range: -50..=50,
    };
    let mut itr = input.iter()
        .map(|r| r.intersection(&bounds))
        .filter(|r| r.is_some())
        .map(|r| r.unwrap());
    let space = build_space(&mut itr);
    space.iter()
        .filter(|s| s.on)
        .map(|s| s.rect.volume())
        .sum()
}

fn part2(input: &Vec<Region>) -> Volume {
    let mut itr = input.iter().cloned();
    let space = build_space(&mut itr);
    space.iter()
        .filter(|s| s.on)
        .map(|s| s.rect.volume())
        .sum()
}

fn main() {
    let input: Vec<Region> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use advent2021::read::test_input;
    use super::*;

    #[test]
    fn day22_test() {
        let input: Vec<Region> = test_input(include_str!("day22.test1input"));
        assert_eq!(part1(&input), 39);
        let input: Vec<Region> = test_input(include_str!("day22.test2input"));
        assert_eq!(part1(&input), 590784);
        let input: Vec<Region> = test_input(include_str!("day22.test3input"));
        assert_eq!(part2(&input), 2758514936282235);
    }
}
