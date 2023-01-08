use std::vec::Vec;
extern crate advent2021;
use advent2021::read::read_input;

fn count_ones(list: &Vec<String>) -> Vec<usize> {
    let nbits = list[0].len();
    list
        .iter()
        .fold(vec![0usize; nbits], |acc, row| {
            let mut v = acc.clone();
            for (idx, c) in row.chars().enumerate() {
                if c == '1' {
                    v[idx] += 1;
                }
            }
            v
        })
}

fn part1(input: &Vec<String>) -> usize {
    let halfrows = input.len() / 2;
    let onecounts = count_ones(input);

    let gamma_bin: String = onecounts
        .iter()
        .map(|b|
            if *b > halfrows {
                '1'
            } else if *b < halfrows {
                '0'
            } else {
                panic!()
            }
        )
        .collect();
    let epsilon_bin: String = onecounts
        .iter()
        .map(|b|
            if *b > halfrows {
                '0'
            } else if *b < halfrows {
                '1'
            } else {
                panic!()
            }
        )
        .collect();
    usize::from_str_radix(&gamma_bin, 2).unwrap() *
        usize::from_str_radix(&epsilon_bin, 2).unwrap()
}

fn get_rating(input: &Vec<String>, o2: bool) -> String {
    let nbits = input[0].len();
    let mut list = input.clone();
    for idx in 0..nbits {
        let counts = count_ones(&list);
        let half = list.len() / 2 + list.len() % 2;
        let match_char = if o2 {
            if counts[idx] >= half { b'1' } else { b'0' }
        } else {
            if counts[idx] < half { b'1' } else { b'0' }
        };
        list = list
            .iter()
            .filter(|row| row.as_bytes()[idx] == match_char)
            .cloned()
            .collect();
        if list.len() == 1 {
            return list[0].clone();
        }
    }
    panic!();
}

fn part2(input: &Vec<String>) -> usize {
    let o2_str = get_rating(input, true);
    let co2_str = get_rating(input, false);
    usize::from_str_radix(&o2_str, 2).unwrap() *
        usize::from_str_radix(&co2_str, 2).unwrap()
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use advent2021::read::test_input;
    use super::*;

    #[test]
    fn day03_test() {
        let input: Vec<String> = test_input(include_str!("day03.testinput"));
        assert_eq!(part1(&input), 198);
        assert_eq!(part2(&input), 230);
    }
}
