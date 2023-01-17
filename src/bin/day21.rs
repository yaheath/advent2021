use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
extern crate advent2021;
use advent2021::read::read_input;

struct Input(i64);

type Pos = i64;
type Score = i64;

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words:Vec<&str> = s.split_whitespace().collect();
        let val = words[4].parse::<Pos>().unwrap();
        Ok(Input(val))
    }
}

fn run_deterministic_game(p1_initial_pos: Pos, p2_initial_pos: Pos) -> i64 {
    let mut d100 = (1..=100).cycle();
    let mut p1_pos = p1_initial_pos;
    let mut p2_pos = p2_initial_pos;
    let mut p1_score: Score = 0;
    let mut p2_score: Score = 0;
    let mut turn = 1;
    let mut rolls = 0i64;
    while p1_score < 1000 && p2_score < 1000 {
        let roll = d100.next().unwrap() + d100.next().unwrap() + d100.next().unwrap();
        rolls += 3;
        if turn == 1 {
            p1_pos = (((p1_pos - 1) + roll) % 10) + 1;
            p1_score += p1_pos;
            turn = 2;
        } else {
            p2_pos = (((p2_pos - 1) + roll) % 10) + 1;
            p2_score += p2_pos;
            turn = 1;
        }
    }
    //println!("rolls: {rolls} p1: {p1_score} p2: {p2_score}");
    rolls * p1_score.min(p2_score)
}

fn part1(input: &Vec<Input>) -> i64 {
    run_deterministic_game(input[0].0, input[1].0)
}

                             // 3  4  5  6  7  8  9
const ROLL_COUNT: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];
const TARGET_SCORE: Score = 21;

fn calc_scores(initial_pos: Pos) -> Vec<HashMap<(Pos, Score), usize>> {
    let mut turn: usize = 0;
    let mut scores: Vec<HashMap<(Pos, Score), usize>> = Vec::new();
    scores.push(HashMap::from_iter([ ((initial_pos, 0), 1) ]));
    let mut done = false;
    while !done {
        turn += 1;
        let mut map = HashMap::new();
        done = true;
        for ((prev_pos, prev_score), prev_count) in scores[turn - 1]
            .iter()
            .filter(|((_, s), _)| *s < TARGET_SCORE)
        {
            for (roll, roll_count) in ROLL_COUNT.iter().enumerate().map(|(i, c)| (i as i64 + 3, c)) {
                let pos = (((prev_pos - 1) + roll) % 10) + 1;
                let score = prev_score + pos;
                if score < TARGET_SCORE {
                    done = false;
                }
                map.entry((pos, score))
                    .and_modify(|count| *count += prev_count * roll_count)
                    .or_insert(prev_count * roll_count);
            }
        }
        scores.push(map);
    }
    scores
}

fn run_dirac_game(p1_initial_pos: Pos, p2_initial_pos: Pos) -> usize {
    let scores_p1 = calc_scores(p1_initial_pos);
    let scores_p2 = calc_scores(p2_initial_pos);

    let target_reached_per_turn_p1: Vec<usize> = scores_p1
        .iter()
        .map(|map| map
            .iter()
            .filter(|((_, score), _)| *score >= TARGET_SCORE)
            .map(|(_, count)| count)
            .sum()
        )
        .collect();

    let target_not_reached_per_turn_p1: Vec<usize> = scores_p1
        .iter()
        .map(|map| map
            .iter()
            .filter(|((_, score), _)| *score < TARGET_SCORE)
            .map(|(_, count)| count)
            .sum()
        )
        .collect();

    let target_reached_per_turn_p2: Vec<usize> = scores_p2
        .iter()
        .map(|map| map
            .iter()
            .filter(|((_, score), _)| *score >= TARGET_SCORE)
            .map(|(_, count)| count)
            .sum()
        )
        .collect();

    let target_not_reached_per_turn_p2: Vec<usize> = scores_p2
        .iter()
        .map(|map| map
            .iter()
            .filter(|((_, score), _)| *score < TARGET_SCORE)
            .map(|(_, count)| count)
            .sum()
        )
        .collect();

    let p1_wins: usize = target_reached_per_turn_p1
        .iter()
        .enumerate()
        .filter(|(turn, _)| *turn > 0 && *turn <= target_not_reached_per_turn_p2.len())
        .map(|(turn, count)| count * target_not_reached_per_turn_p2[turn - 1])
        .sum();

    let p2_wins = target_reached_per_turn_p2
        .iter()
        .enumerate()
        .filter(|(turn, _)| *turn < target_not_reached_per_turn_p1.len())
        .map(|(turn, count)| count * target_not_reached_per_turn_p1[turn])
        .sum();

    // println!("p1: {p1_wins} p2: {p2_wins}");
    p1_wins.max(p2_wins)
}

fn part2(input: &Vec<Input>) -> usize {
    run_dirac_game(input[0].0, input[1].0)
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use advent2021::read::test_input;
    use super::*;

    #[test]
    fn day21_test() {
        let input: Vec<Input> = test_input("Player 1 starting position: 4\nPlayer 2 starting position: 8");
        assert_eq!(part1(&input), 739785);
        assert_eq!(part2(&input), 444356092776315);
    }
}
