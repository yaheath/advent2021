use std::vec::Vec;
extern crate advent2021;
use advent2021::read::read_input;

/*
fn sim(input: &Vec<u8>, iterations: usize) -> usize {
    let mut fish = input.clone();

    for _ in 0..iterations {
        let mut add: usize = 0;
        fish.iter_mut().for_each(|f| match *f {
            0 => { *f = 6; add += 1; },
            _ => { *f -= 1; },
        });
        for _ in 0..add {
            fish.push(8);
        }
    }
    fish.len()
}
*/

fn sim_scalable(input: &Vec<u8>, iterations: usize) -> usize {
    let mut fish_by_phase: [usize; 7] = [0; 7];
    let mut pending_by_phase: [usize; 7] = [0; 7];

    for f in input {
        fish_by_phase[*f as usize + 1] += 1;
    }
    for i in 0..=iterations {
        let cur_phase = i % 7;
        let pend_phase = (i + 2) % 7;
        pending_by_phase[pend_phase] += fish_by_phase[cur_phase];
        fish_by_phase[cur_phase] += pending_by_phase[cur_phase];
        pending_by_phase[cur_phase] = 0;
    }
    fish_by_phase.iter().sum::<usize>() + pending_by_phase.iter().sum::<usize>()
}

fn part1(input: &Vec<u8>) -> usize {
    sim_scalable(&input, 80)
}

fn part2(input: &Vec<u8>) -> usize {
    sim_scalable(&input, 256)
}

fn main() {
    let input: Vec<String> = read_input();
    let fish: Vec<u8> = input[0].split(',').map(|n| n.parse::<u8>().unwrap()).collect();
    println!("Part 1: {}", part1(&fish));
    println!("Part 2: {}", part2(&fish));
}

#[cfg(test)]
mod tests {
    use advent2021::read::test_input;
    use super::*;

    #[test]
    fn day06_test() {
        let input: Vec<String> = test_input("3,4,3,1,2");
        let fish: Vec<u8> = input[0].split(',').map(|n| n.parse::<u8>().unwrap()).collect();
        assert_eq!(part1(&fish), 5934);
        assert_eq!(part2(&fish), 26984457539);
    }
}
