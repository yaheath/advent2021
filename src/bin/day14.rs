use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use ya_advent_lib::read::read_input;

enum Input {
    Template(String),
    Rule((char, char), char),
    Blank,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Input::Blank)
        } else if s.contains(" -> ") {
            let mut itr = s.split(" -> ");
            let one = itr.next().unwrap();
            let two = itr.next().unwrap();
            let mut cc = one.chars();
            Ok(Input::Rule(
                (cc.next().unwrap(), cc.next().unwrap()),
                two.chars().next().unwrap(),
            ))
        }
        else {
            Ok(Input::Template(s.into()))
        }
    }
}

fn setup(input: &[Input]) -> (String, HashMap<(char, char), char>) {
    let mut template = String::new();
    let mut rules = HashMap::new();
    for i in input {
        match i {
            Input::Blank => {},
            Input::Template(s) => { template = s.clone(); },
            Input::Rule(k, v) => { rules.insert(*k, *v); },
        }
    }
    (template, rules)
}

fn run(input: &[Input], iters: i64) -> i64 {
    let (template, rules) = setup(input);
    let mut pairs: HashMap<(char, char), i64> = HashMap::new();
    let mut histogram: HashMap<char, i64> = HashMap::new();
    template
        .chars()
        .map(|c| {
            histogram.entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            c
        })
        .tuple_windows()
        .for_each(|w| {
            pairs
                .entry(w)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });
    for _ in 0..iters {
        let mut newpairs: HashMap<(char, char), i64> = HashMap::new();
        for (p, count) in &pairs {
            let c = rules[&p];
            histogram
                .entry(c)
                .and_modify(|x| *x += count)
                .or_insert(*count);
            newpairs
                .entry((p.0, c))
                .and_modify(|x| *x += count)
                .or_insert(*count);
            newpairs
                .entry((c, p.1))
                .and_modify(|x| *x += count)
                .or_insert(*count);
            newpairs
                .entry(*p)
                .and_modify(|x| *x -= count)
                .or_insert(-*count);
        }
        for (p, count) in newpairs {
            pairs
                .entry(p)
                .and_modify(|x| *x += count)
                .or_insert(count);
        }
    }
    match histogram.values().minmax() {
        MinMax(min, max) => *max - *min,
        _ => panic!(),
    }
}

fn part1(input: &[Input]) -> i64 {
    run(input, 10)
}

fn part2(input: &[Input]) -> i64 {
    run(input, 40)
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use ya_advent_lib::read::test_input;
    use super::*;

    #[test]
    fn day14_test() {
        let input: Vec<Input> = test_input(include_str!("day14.testinput"));
        assert_eq!(part1(&input), 1588);
        assert_eq!(part2(&input), 2188189693529);
    }
}
