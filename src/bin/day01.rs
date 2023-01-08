use std::vec::Vec;
use itertools::Itertools;
extern crate advent2021;
use advent2021::read::read_input;

fn part1(input: &Vec<i32>) -> usize {
    input
        .windows(2)
        .filter(|slice| slice[1] > slice[0])
        .count()
}

fn part2(input: &Vec<i32>) -> usize {
    input
        .windows(3)
        .map(|slice| slice.iter().sum::<i32>())
        .tuple_windows::<(_,_)>()
        .filter(|(a,b)| b > a)
        .count()
}

fn main() {
    let input: Vec<i32> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use advent2021::read::test_input;
    use super::*;

    #[test]
    fn day01_test() {
        let input: Vec<i32> = test_input(include_str!("day01.testinput"));
        assert_eq!(part1(&input), 7);
        assert_eq!(part2(&input), 5);
    }
}
