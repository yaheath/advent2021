use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_grouped_input;

enum Input {
    Draws(Vec<u32>),
    BoardRow(Vec<u32>)
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(',') {
            Ok(
                Input::Draws(
                    s.split(',')
                    .map(|sl| sl.parse::<u32>().unwrap())
                    .collect()
                )
            )
        }
        else {
            Ok(
                Input::BoardRow(
                    s.split_whitespace()
                    .map(|sl| sl.parse::<u32>().unwrap())
                    .collect()
                )
            )
        }
    }
}


struct Board {
    numbers: [u32; 25],
    markers: [bool; 25],
}

impl Board {
    fn new(input: &Vec<Input>) -> Self {
        let mut arr: [u32; 25] = [0; 25];
        let input: Vec<&Vec<u32>> = input
            .iter()
            .map(|r| match r { Input::BoardRow(row) => row, _ => panic!() })
            .collect();
        for row in 0..5 {
            for col in 0..5 {
                arr[row * 5 + col] = input[row][col];
            }
        }
        Self {
            numbers: arr,
            markers: [false; 25],
        }
    }

    fn mark(&mut self, number: u32) -> bool {
        if let Some((idx, _)) = self.numbers
            .iter()
            .enumerate()
            .find(|(_,n)| **n == number)
        {
                self.markers[idx] = true;
                self.is_winner()
        } else {
            false
        }
    }

    fn is_winner(&self) -> bool {
        for row in 0..5 {
            let mut complete = true;
            for col in 0..5 {
                if !self.markers[row * 5 + col] {
                    complete = false;
                    break;
                }
            }
            if complete { return true; }
        }
        for col in 0..5 {
            let mut complete = true;
            for row in 0..5 {
                if !self.markers[row * 5 + col] {
                    complete = false;
                    break;
                }
            }
            if complete { return true; }
        }
        false
    }

    fn sum_unmarked(&self) -> u32 {
        self.numbers
            .iter()
            .enumerate()
            .fold(0, |sum, (idx, val)| {
                if self.markers[idx] {
                    sum
                } else {
                    sum + val
                }
            })
    }
}

fn split_input(input: &Vec<Vec<Input>>) -> (Vec<u32>, Vec<Board>) {
    if let Input::Draws(draws) = &(input[0])[0] {
        let boards: Vec<Board> = input
            .iter()
            .skip(1)
            .map(|v| Board::new(v))
            .collect();
        (draws.clone(), boards)
    }
    else {
        panic!("expected Moves");
    }
}

fn part1(input: &Vec<Vec<Input>>) -> u32 {
    let (draws, mut boards) = split_input(input);
    for m in draws {
        for b in &mut boards {
            if b.mark(m) {
                return b.sum_unmarked() * m;
            }
        }
    }
    panic!();
}

fn part2(input: &Vec<Vec<Input>>) -> u32 {
    let (draws, mut boards) = split_input(input);
    let mut last = 0u32;
    for m in draws {
        for b in &mut boards {
            if b.is_winner() {
                continue;
            }
            if b.mark(m) {
                last = b.sum_unmarked() * m;
            }
        }
    }
    last
}

fn main() {
    let input: Vec<Vec<Input>> = read_grouped_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use advent_lib::read::grouped_test_input;
    use super::*;

    #[test]
    fn day04_test() {
        let input: Vec<Vec<Input>> = grouped_test_input(include_str!("day04.testinput"));
        assert_eq!(part1(&input), 4512);
        assert_eq!(part2(&input), 1924);
    }
}
