use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use advent_lib::read::read_input;

struct Edge {
    a: String,
    b: String,
}

impl FromStr for Edge {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splt = s.split('-');
        Ok(Edge{
            a: splt.next().unwrap().into(),
            b: splt.next().unwrap().into(),
        })
    }
}

struct Cave {
    is_large: bool,
    neighbors: HashSet<String>,
}

impl Cave {
    fn new(name: &str, neighbor: &str) -> Self {
        let mut neighbors = HashSet::new();
        neighbors.insert(neighbor.to_owned());
        Self {
            is_large: match name.chars().next().unwrap() {
                'A'..='Z' => true,
                _ => false,
            },
            neighbors,
        }
    }
}

fn mkcaves(input: &[Edge]) -> HashMap<String, Cave> {
    let mut caves: HashMap<String, Cave> = HashMap::new();
    for edge in input {
        caves.entry(edge.a.clone())
            .and_modify(|c| {c.neighbors.insert(edge.b.clone());})
            .or_insert(Cave::new(&edge.a, &edge.b));
        caves.entry(edge.b.clone())
            .and_modify(|c| {c.neighbors.insert(edge.a.clone());})
            .or_insert(Cave::new(&edge.b, &edge.a));
    }
    caves
}

fn traverse(current: &str, caves: &HashMap<String, Cave>, visited: &HashSet<String>, repeat: Option<&str>) -> Vec<String> {
    if current == "end" {
        return vec!["end".into()];
    }
    let mut visited = visited.clone();
    let mut repeat = repeat.clone();
    match repeat {
        Some(r) => {
            if r == current {
                repeat = None;
            } else {
                visited.insert(current.to_owned());
            }
        }
        None => {
            visited.insert(current.to_owned());
        }
    }
    let cave = &caves[current];
    let mut paths = Vec::new();
    for n in &cave.neighbors {
        let neighbor = &caves[n];
        if neighbor.is_large || !visited.contains(n) {
            for p in traverse(&n, caves, &visited, repeat.clone()) {
                paths.push(format!("{current},{}", p));
            }
        }
    }
    paths
}

fn part1(caves: &HashMap<String, Cave>) -> usize {
    let visited: HashSet<String> = HashSet::new();
    traverse("start", caves, &visited, None).len()
}

fn part2(caves: &HashMap<String, Cave>) -> usize {
    let visited: HashSet<String> = HashSet::new();
    caves
        .iter()
        .filter(|(k,v)| !v.is_large && *k != "start" && *k != "end")
        .flat_map(|(k,_)| traverse("start", caves, &visited, Some(&k)))
        .unique()
        .count()
}

fn main() {
    let input: Vec<Edge> = read_input();
    let caves = mkcaves(&input);
    println!("Part 1: {}", part1(&caves));
    println!("Part 2: {}", part2(&caves));
}

#[cfg(test)]
mod tests {
    use advent_lib::read::test_input;
    use super::*;

    #[test]
    fn day12_test() {
        let input1: Vec<Edge> = test_input(include_str!("day12.test1input"));
        let caves1 = mkcaves(&input1);
        assert_eq!(part1(&caves1), 10);
        assert_eq!(part2(&caves1), 36);
        let input2: Vec<Edge> = test_input(include_str!("day12.test2input"));
        let caves2 = mkcaves(&input2);
        assert_eq!(part1(&caves2), 19);
        assert_eq!(part2(&caves2), 103);
        let input3: Vec<Edge> = test_input(include_str!("day12.test3input"));
        let caves3 = mkcaves(&input3);
        assert_eq!(part1(&caves3), 226);
        assert_eq!(part2(&caves3), 3509);
    }
}
