use std::collections::{HashMap,HashSet};
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_grouped_input;

type Coord = (i64,i64,i64);
enum Input {
    Header,
    Coord(Coord),
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(',') {
            let mut itr = s.split(',');
            let x = itr.next().unwrap().parse::<i64>().unwrap();
            let y = itr.next().unwrap().parse::<i64>().unwrap();
            let z = itr.next().unwrap().parse::<i64>().unwrap();
            Ok(Input::Coord((x,y,z)))
        } else {
            Ok(Input::Header)
        }
    }
}

#[derive(Clone)]
struct Scanner {
    id: usize,
    beacons: Vec<Coord>,
    relative_beacons: HashSet<Coord>,
    loc: Coord,
}

#[derive(Debug, Clone)]
struct Transform {
    offset: Coord,
    rotation: usize,    //index into rotations() vec
}

fn rotations(coord:Coord) -> Vec<Coord> {
    let (x, y, z) = coord;
    vec![
        (x, y, z),   (x, z, -y), (x, -y, -z), (x, -z, y),
        (-x, -y, z), (-x, z, y), (-x, y, -z), (-x, -z, -y),
        (y, z, x),   (y, x, -z), (y, -z, -x), (y, -x, z),
        (-y, -z, x), (-y, x, z), (-y, z, -x), (-y, -x, -z),
        (z, x, y),   (z, y, -x), (z, -x, -y), (z, -y, x),
        (-z, -x, y), (-z, y, x), (-z, x, -y), (-z, -y, -x),
    ]
}

impl Scanner {
    fn new(beacons: Vec<Coord>, id: usize, loc: Coord) -> Self {
        let relative_beacons = beacons
            .iter()
            .tuple_combinations()
            .flat_map(|(a,b)| [ (a.0-b.0,a.1-b.1,a.2-b.2), (b.0-a.0,b.1-a.1,b.2-a.2) ])
            .collect();

        Self {
            id,
            beacons,
            relative_beacons,
            loc,
        }
    }

    // try to match beacons between self and other. if enough
    // beacons match up, return the transform that maps from
    // self's orientation to other. The problem statement says
    // there will be at least 12 beacons visible between pairs
    // of scanners (that can be matched).
    fn match_beacons(&self, other: &Scanner) -> Option<Transform> {
        let mut found_rot:Option<usize> = None;
        for rot in 0..24 {
            let other_rot:HashSet<Coord> =
                other.relative_beacons.iter().map(|o| rotations(*o)[rot]).collect();
            let common = self.relative_beacons.intersection(&other_rot).count();
            // there should be 132 common relative pairs for 12 common beacons
            if common >= 132 {
                found_rot = Some(rot);
                break;
            }
        }
        found_rot?;
        let rotation = found_rot.unwrap();
        let other_beacons:Vec<Coord> = other.beacons
            .iter()
            .map(|c| rotations(*c)[rotation])
            .collect();

        let hist = self.beacons
            .iter()
            .map(|b| (0i64, b))
            .chain(
                other_beacons.iter().map(|b| (1i64, b))
            )
            .tuple_combinations()
            .filter(|(a,b)| a.0 != b.0)
            .map(|((ka,a),(_,b))|
                if ka == 0 {
                    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
                } else {
                    (b.0 - a.0, b.1 - a.1, b.2 - a.2)
                }
            )
            .fold(
                HashMap::new(),
                |mut map, elem| {
                    map.entry(elem)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    map
                }
            );
        let (offset, _) = hist.iter().max_by(|a,b| a.1.cmp(b.1)).unwrap();
        Some(Transform {
            offset: *offset,
            rotation,
        })
    }

    fn transformed(&self, xf: &Transform) -> Scanner {
        Scanner::new(
            self.beacons
                .iter()
                .map(|b| rotations(*b)[xf.rotation])
                .map(|b| (b.0 + xf.offset.0, b.1 + xf.offset.1, b.2 + xf.offset.2))
                .collect(),
            self.id,
            (self.loc.0 + xf.offset.0, self.loc.1 + xf.offset.1, self.loc.2 + xf.offset.2),
        )
    }
}

fn setup(input: &[Vec<Input>]) -> Vec<Scanner> {
    input
        .iter()
        .map(|group| group
            .iter()
            .filter(|i| !matches!(i, Input::Header))
            .map(|i| match i { Input::Coord(p) => *p, _ => panic!() })
            .collect::<Vec<Coord>>()
        )
        .enumerate()
        .map(|(idx, v)| Scanner::new(v, idx, (0,0,0)))
        .collect()
}

fn construct_space(scanners: &Vec<Scanner>) -> Vec<Scanner> {
    let mut matched: HashMap<usize,Vec<usize>> = HashMap::new();
    for (s1, s2) in scanners
            .iter()
            .tuple_combinations()
            .map(|(s1,s2)| (s1, s2, s1.match_beacons(s2)))
            .filter(|(_,_,m)| m.is_some())
            .flat_map(|(s1,s2,_)| [(s1.id, s2.id), (s2.id, s1.id)] )
    {
        matched.entry(s1)
            .and_modify(|v| v.push(s2))
            .or_insert(vec![s2]);
    }
    let mut stack: Vec<usize> = Vec::new();
    let mut processed: HashMap<usize,Scanner> = HashMap::new();
    processed.insert(0,scanners[0].clone());
    matched[&0].iter().for_each(|other| {
        stack.push(*other);
        let xfrm = scanners[0].match_beacons(&scanners[*other]).unwrap();
        let new = scanners[*other].transformed(&xfrm);
        processed.insert(*other, new);
    });
    while let Some(idx) = stack.pop() {
        matched[&idx].iter()
            .for_each(|other| {
                if !processed.contains_key(other) {
                    stack.push(*other);
                    let xfrm = processed[&idx].match_beacons(&scanners[*other]).unwrap();
                    let new = scanners[*other].transformed(&xfrm);
                    processed.insert(*other, new);
                }
            });
    }
    let mut out = Vec::new();
    for i in 0..scanners.len() {
        out.push(processed.remove(&i).unwrap());
    }
    out
}

fn part1(scanners: &[Scanner]) -> usize {
    let set: HashSet<_> = scanners
        .iter()
        .flat_map(|s| s.beacons.iter())
        .collect();
    set.len()
}

fn part2(scanners: &[Scanner]) -> i64 {
    scanners
        .iter()
        .tuple_combinations()
        .map(|(a,b)| (a.loc.0 - b.loc.0).abs() + (a.loc.1 - b.loc.1).abs() + (a.loc.2 - b.loc.2).abs())
        .max()
        .unwrap()
}

fn main() {
    let input: Vec<Vec<Input>> = read_grouped_input();
    let scanners: Vec<Scanner> = setup(&input);
    let scanners = construct_space(&scanners);
    println!("Part 1: {}", part1(&scanners));
    println!("Part 2: {}", part2(&scanners));
}

#[cfg(test)]
mod tests {
    use ya_advent_lib::read::grouped_test_input;
    use super::*;

    #[test]
    fn day19_test() {
        let input: Vec<Vec<Input>> = grouped_test_input(include_str!("day19.testinput"));
        let scanners: Vec<Scanner> = setup(&input);
        let scanners = construct_space(&scanners);
        assert_eq!(part1(&scanners), 79);
        assert_eq!(part2(&scanners), 3621);
    }
}
