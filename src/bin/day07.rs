use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn part1(input: &[i32]) -> i32 {
    let minval = *input.iter().min().unwrap();
    let maxval = *input.iter().max().unwrap();
    let mut minfuel = i32::MAX;
    for target in minval..=maxval {
        minfuel = input
            .iter()
            .map(|v| (*v - target).abs())
            .sum::<i32>()
            .min(minfuel);
    }
    minfuel
}

fn part2(input: &[i32]) -> i32 {
    let minval = *input.iter().min().unwrap();
    let maxval = *input.iter().max().unwrap();
    let mut minfuel = i32::MAX;
    for target in minval..=maxval {
        minfuel = input
            .iter()
            .map(|v| {
                let dist = (*v - target).abs();
                (dist * (dist + 1)) / 2
            })
            .sum::<i32>()
            .min(minfuel);
    }
    minfuel
}

fn main() {
    let input: Vec<String> = read_input();
    let subs: Vec<i32> = input[0].split(',').map(|n| n.parse::<i32>().unwrap()).collect();
    println!("Part 1: {}", part1(&subs));
    println!("Part 2: {}", part2(&subs));
}

#[cfg(test)]
mod tests {
    use ya_advent_lib::read::test_input;
    use super::*;

    #[test]
    fn day06_test() {
        let input: Vec<String> = test_input("16,1,2,0,4,2,7,1,2,14");
        let subs: Vec<i32> = input[0].split(',').map(|n| n.parse::<i32>().unwrap()).collect();
        assert_eq!(part1(&subs), 37);
        assert_eq!(part2(&subs), 168);
    }
}
