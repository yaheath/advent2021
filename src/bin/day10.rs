use std::vec::Vec;
use advent_lib::read::read_input;

enum LineError {
    Corrupt(char),
    Missing(String),
}

impl LineError {
    fn corrupt(&self) -> Option<char> {
        match self {
            LineError::Corrupt(c) => Some(*c),
            _ => None,
        }
    }
    fn missing(&self) -> Option<&String> {
        match self {
            LineError::Missing(s) => Some(s),
            _ => None,
        }
    }
}

fn analyze_line(line: &str) -> LineError {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        let top = if stack.is_empty() { ' ' } else { stack[stack.len() - 1] };
        match c {
            '<' => { stack.push('>'); },
            '[' => { stack.push(']'); },
            '(' => { stack.push(')'); },
            '{' => { stack.push('}'); },
            '>' | ']' | '}' | ')' => {
                if top != c {
                    return LineError::Corrupt(c);
                }
                stack.pop();
            },
            _ => panic!(),
        }
    }
    LineError::Missing(stack.iter().rev().collect())
}

fn score_corrupt(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn score_missing(s: &str) -> usize {
    s.chars()
        .fold(0, |acc, c| acc * 5 + match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!(),
        })
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|s| analyze_line(s))
        .filter(|opt| opt.corrupt().is_some())
        .map(|opt| score_corrupt(opt.corrupt().unwrap()))
        .sum()
}

fn part2(input: &[String]) -> usize {
    let mut scores: Vec<usize> = input
        .iter()
        .map(|s| analyze_line(s))
        .filter(|opt| opt.missing().is_some())
        .map(|opt| score_missing(opt.missing().unwrap()))
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use advent_lib::read::test_input;
    use super::*;

    #[test]
    fn day10_test() {
        let input: Vec<String> = test_input(include_str!("day10.testinput"));
        assert_eq!(part1(&input), 26397);
        assert_eq!(part2(&input), 288957);
    }
}
