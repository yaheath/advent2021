#[macro_use] extern crate lazy_static;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
use ya_advent_lib::read::read_input;

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+) (\d+)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let val = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            match caps.get(1).unwrap().as_str() {
                "forward" => Ok(Command::Forward(val)),
                "up" => Ok(Command::Up(val)),
                "down" => Ok(Command::Down(val)),
                _ => Err(())
            }
        }
        else {
            Err(())
        }
    }
}

fn part1(input: &[Command]) -> i32 {
    let (depth, horiz) = input
        .iter()
        .fold((0, 0), |(depth, horiz), cmd| {
            match cmd {
                Command::Forward(n) => (depth, horiz + n),
                Command::Up(n) => (depth - n, horiz),
                Command::Down(n) => (depth + n, horiz),
            }
        });

    depth * horiz
}

fn part2(input: &[Command]) -> i32 {
    let (depth, horiz, _) = input
        .iter()
        .fold((0, 0, 0), |(depth, horiz, aim), cmd| {
            match cmd {
                Command::Forward(n) => (depth + aim * n, horiz + n, aim),
                Command::Up(n) => (depth, horiz, aim - n),
                Command::Down(n) => (depth, horiz, aim + n),
            }
        });

    depth * horiz
}

fn main() {
    let input: Vec<Command> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use ya_advent_lib::read::test_input;
    use super::*;

    #[test]
    fn day02_test() {
        let input: Vec<Command> = test_input(include_str!("day02.testinput"));
        assert_eq!(part1(&input), 150);
        assert_eq!(part2(&input), 900);
    }
}
