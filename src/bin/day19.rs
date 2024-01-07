use std::collections::{HashMap,HashSet};
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::coords::Coord3D;
use ya_advent_lib::read::read_grouped_input;

enum Input {
    Header,
    Coord(Coord3D),
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(',') {
            let c = s.parse::<Coord3D>().unwrap();
            Ok(Input::Coord(c))
        } else {
            Ok(Input::Header)
        }
    }
}

#[derive(Clone)]
struct Scanner {
    id: usize,
    beacons: Vec<Coord3D>,
    relative_beacons: HashSet<Coord3D>,
    loc: Coord3D,
}

#[derive(Debug, Clone)]
struct Transform {
    offset: Coord3D,
    rotation: usize,    //index into rotations() vec
}

fn rotations(coord:Coord3D) -> Vec<Coord3D> {
    let (x, y, z) = (coord.x, coord.y, coord.z);
    let n = |a,b,c| Coord3D::new(a,b,c);
    vec![
        n(x, y, z),   n(x, z, -y), n(x, -y, -z), n(x, -z, y),
        n(-x, -y, z), n(-x, z, y), n(-x, y, -z), n(-x, -z, -y),
        n(y, z, x),   n(y, x, -z), n(y, -z, -x), n(y, -x, z),
        n(-y, -z, x), n(-y, x, z), n(-y, z, -x), n(-y, -x, -z),
        n(z, x, y),   n(z, y, -x), n(z, -x, -y), n(z, -y, x),
        n(-z, -x, y), n(-z, y, x), n(-z, x, -y), n(-z, -y, -x),
    ]
}

impl Scanner {
    fn new(beacons: Vec<Coord3D>, id: usize, loc: Coord3D) -> Self {
        let relative_beacons = beacons
            .iter()
            .tuple_combinations()
            .flat_map(|(a,b)| [ *a - *b, *b - *a ])
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
            let other_rot:HashSet<Coord3D> =
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
        let other_beacons:Vec<Coord3D> = other.beacons
            .iter()
            .map(|c| rotations(*c)[rotation])
            .collect();

        let hist = self.beacons
            .iter()
            .map(|b| (0, b))
            .chain(
                other_beacons.iter().map(|b| (1, b))
            )
            .tuple_combinations()
            .filter(|(a,b)| a.0 != b.0)
            .map(|((ka,a),(_,b))|
                if ka == 0 {
                    *a - *b
                } else {
                    *b - *a
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
                .map(|b| b + xf.offset)
                .collect(),
            self.id,
            self.loc + xf.offset,
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
            .collect::<Vec<Coord3D>>()
        )
        .enumerate()
        .map(|(idx, v)| Scanner::new(v, idx, Coord3D::new(0,0,0)))
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
        .map(|(a,b)| a.loc.mdist_to(&b.loc))
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
