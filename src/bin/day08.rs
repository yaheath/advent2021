use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::vec::Vec;
extern crate advent2021;
use advent2021::read::read_input;

#[derive(Copy,Clone,Eq,PartialEq,Hash)]
enum Seg {
    A, B, C, D, E, F, G,
}

#[derive(Clone,Eq,PartialEq)]
struct SSeg {
    segs: HashSet<Seg>,
}

impl Hash for SSeg {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_string().hash(state);
    }
}

impl SSeg {
    fn new(chars: &str) -> Self {
        Self {
          segs: chars.chars().map(|c| match c {
                'a'=>Seg::A, 'b'=>Seg::B, 'c'=>Seg::C, 'd'=>Seg::D,
                'e'=>Seg::E, 'f'=>Seg::F, 'g'=>Seg::G, _=>panic!(),
            }).collect(),
        }
    }
    fn val_by_len(&self) -> Option<u8> {
        match self.segs.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }
    fn as_string(&self) -> String {
        let mut chars: Vec<char> = self.segs
            .iter()
            .map(|s| match s {
                Seg::A=>'a', Seg::B=>'b', Seg::C=>'c', Seg::D=>'d',
                Seg::E=>'e', Seg::F=>'f', Seg::G=>'g',
            })
            .collect();
        chars.sort();
        chars.iter().collect()
    }
}

struct Disp {
    patterns: Vec<SSeg>,
    displays: Vec<SSeg>,
}

impl FromStr for Disp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr1 = s.split(" | ");
        let patterns = itr1.next().unwrap().split(' ').map(|d| SSeg::new(d)).collect();
        let displays = itr1.next().unwrap().split(' ').map(|d| SSeg::new(d)).collect();
        Ok (
            Disp {
                patterns,
                displays,
            }
        )
    }
}

impl Disp {
    fn solve(&self) -> usize {
        let mut digmap: HashMap<u8, SSeg> =
            self.patterns
                .iter()
                .map(|p| (p, p.val_by_len()))
                .filter(|(_, v)| v.is_some())
                .map(|(p, v)| (v.unwrap(), p.clone()))
                .collect();
        // digmap now has 1, 4, 7, and 8

        let mut unknowns: HashSet<SSeg> =
            self.patterns
                .iter()
                .map(|p| (p, p.val_by_len()))
                .filter(|(_, v)| v.is_none())
                .map(|(p, _)| p.clone())
                .collect();

        // find the 9 which is the only number besides 8 that has the segments
        // from 4|7
        let abcdf:HashSet<Seg> = digmap[&4].segs
            .union(&digmap[&7].segs)
            .cloned().collect();

        let nine = unknowns
            .iter()
            .filter(|p| p.segs.is_superset(&abcdf))
            .next()
            .unwrap()
            .clone();

        digmap.insert(9, nine.clone());
        unknowns.remove(&nine);
        let gseg = nine.segs.difference(&abcdf).next().unwrap().clone();

        // the only digits which have ACFG (that isn't 8 or 9) are 0 and 3
        let mut acfg = digmap[&7].segs.clone();
        acfg.insert(gseg);
        let zero_or_three: Vec<SSeg> = unknowns
            .iter()
            .filter(|p| p.segs.is_superset(&acfg))
            .cloned()
            .collect();
        assert_eq!(zero_or_three.len(), 2);
        if zero_or_three[0].segs.len() == 6 {
            digmap.insert(0, zero_or_three[0].clone());
            digmap.insert(3, zero_or_three[1].clone());
        } else {
            digmap.insert(0, zero_or_three[1].clone());
            digmap.insert(3, zero_or_three[0].clone());
        }
        unknowns.remove(&zero_or_three[0]);
        unknowns.remove(&zero_or_three[1]);

        let be_segs:HashSet<Seg> = digmap[&0].segs.difference(&acfg).cloned().collect();   // B and E segs
        let eseg = be_segs.difference(&digmap[&4].segs).next().unwrap().clone();

        // 6 is the only remaining unknown that has B and E
        let six = unknowns
            .iter()
            .filter(|p| p.segs.is_superset(&be_segs))
            .next()
            .unwrap()
            .clone();

        digmap.insert(6, six.clone());
        unknowns.remove(&six);

        // 2 is the only remaining unknown to have E
        let two = unknowns
            .iter()
            .filter(|p| p.segs.contains(&eseg))
            .next()
            .unwrap()
            .clone();
        digmap.insert(2, two.clone());
        unknowns.remove(&two);

        // should be one left, and thats 5
        assert_eq!(unknowns.len(), 1);
        let five = unknowns.iter().next().unwrap().clone();
        digmap.insert(5, five);

        let rdigmap: HashMap<SSeg,u8> = digmap.iter().map(|(k,v)| (v.clone(), *k)).collect();

        self.displays
            .iter()
            .map(|d| rdigmap[d] as usize)
            .fold(0usize, |acc, v| acc * 10 + v)
    }
}

fn part1(input: &Vec<Disp>) -> usize {
    input
        .iter()
        .map(|row| row.displays
            .iter()
            .filter(|s| s.val_by_len().is_some())
            .count()
        )
        .sum()
}

fn part2(input: &Vec<Disp>) -> usize {
    let list: Vec<usize> = input
        .iter()
        .map(|row| row.solve())
        .collect();
    list.iter().sum()
}

fn main() {
    let input: Vec<Disp> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use advent2021::read::test_input;
    use super::*;

    #[test]
    fn day08_test() {
        let input: Vec<Disp> = test_input(include_str!("day08.testinput"));
        assert_eq!(part1(&input), 26);
        assert_eq!(part2(&input), 61229);
        let input2: Vec<Disp> = test_input("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");
        assert_eq!(part1(&input2), 0);
        assert_eq!(part2(&input2), 5353);
    }
}
