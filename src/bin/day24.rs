use itertools::chain;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use advent_lib::read::input_as_string;

#[derive(Debug, Clone)]
struct Instruction {
    divisor: i32,
    x_increment: i32,
    y_increment: i32,
}

impl Instruction {
    fn parse(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"mul x 0
add x z
mod x 26
div z (26|1)
add x (-?[0-9]+)
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y (-?[0-9]+)
mul y x
add z y"
            )
            .unwrap();
        }

        let caps = RE.captures(input)?;

        // If the regex matches, it should be safe to unwrap
        Some(Self {
            divisor: caps[1].parse().unwrap(),
            x_increment: caps[2].parse().unwrap(),
            y_increment: caps[3].parse().unwrap(),
        })
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .split("inp w")
        .filter(|part| part != &"")
        .map(|part| Instruction::parse(part).unwrap())
        .collect()
}

// This problem is similar to a stack machine, as discussed on the advent of code subreddit. A
// "push" occurs anytime the divisor is 1. A "pop" occurs anytime the divisor is 26. In the latter
// case, w must be equal to z % 26 + x_increment. Therefore, we must push a value in the former
// case to make this satisfiable.
//
// My solution assumes that every push operation has a matching pop operation. This means a push
// could be followed by a pop, two pushes could be folioed by two pops, and so on. The recurse
// function will match pushes to pops, processing any nested operations, and the remaining,
// following operations as appropriate
//
// For example, here are the calls to recurse for my input. The push and pop for one recurse are
// marked, and nested instructions are indented
// Recurse
// ├ { divisor: 1, x_increment: 10, y_increment: 13 }
// │ Recurse
// │ ├ { divisor: 1, x_increment: 13, y_increment: 10 }
// │ │ Recurse
// │ │ ├ { divisor: 1, x_increment: 13, y_increment: 3 }
// │ │ └ { divisor: 26, x_increment: -11, y_increment: 1 }
// │ │ Recurse
// │ │ ├ { divisor: 1, x_increment: 11, y_increment: 9 }
// │ │ └ { divisor: 26, x_increment: -4, y_increment: 3 }
// │ │ Recurse
// │ │ ├ { divisor: 1, x_increment: 12, y_increment: 5 }
// │ │ │ Recurse
// │ │ │ ├ { divisor: 1, x_increment: 12, y_increment: 1 }
// │ │ │ │ Recurse
// │ │ │ │ ├ { divisor: 1, x_increment: 15, y_increment: 0 }
// │ │ │ │ └ { divisor: 26, x_increment: -2, y_increment: 13 }
// │ │ │ └ { divisor: 26, x_increment: -5, y_increment: 7 }
// │ │ └ { divisor: 26, x_increment: -11, y_increment: 15 }
// │ └ { divisor: 26, x_increment: -13, y_increment: 12 }
// └ { divisor: 26, x_increment: -10, y_increment: 8 }
//
// Once pushes and pops are paired, I can calculate the maximum (or minimum, for part 2) value of
// the left digit that satisfies the range of the right digit. This is similar to the stack machine
// that I have seen many discussions about, but uses recursion rather than an actual stack (well,
// it uses the call stack). I first did a solution using a stack, but pushing a value onto the
// stack requires future information, which is why I pair up a push and a pop and calculate the
// value for both at once.

fn recurse(
    instructions: &[Instruction],
    question_part: u8,
) -> (&[Instruction], Vec<i32>) {
    // If we've exhausted input or we get to somebody's right hand side,
    // stop recursing
    if instructions.is_empty() || instructions[0].divisor == 26 {
        return (instructions, vec![]);
    }

    // Get the left instruction (divisor = 1)
    let (left, instructions) = instructions.split_first().unwrap();

    // Parse all the instructions in between this pair
    let (instructions, mid) = recurse(instructions, question_part);

    // Get the right instruction (divisor = 26)
    let (right, instructions) = instructions.split_first().unwrap();

    let left_output = match question_part {
        // Calculate the maximum value the left digit can be without making the right value go over
        // 9
        1 => min(9, 9 - left.y_increment - right.x_increment),
        // Calculate the minimum value the left digit can be in the same way
        2 => max(1, 1 - left.y_increment - right.x_increment),
        _ => panic!(),
    };
    // Calculate right digit based on left digit, left y-increment, and right x-increment
    let right_output = left_output + left.y_increment + right.x_increment;

    // Get digits from the remainder of the input
    let (instructions, tail) = recurse(instructions, question_part);

    // Chain left, mid, right, and remainder
    (
        instructions,
        chain!([left_output], mid, [right_output], tail).collect::<Vec<i32>>(),
    )
}

fn doit(instructions: &Vec<Instruction>, part: u8) -> usize {
    let (instructions, ans) = recurse(&instructions[..], part);
    assert!(instructions.is_empty());

    ans
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (x as usize) * 10_usize.pow(i as u32))
}

fn main() {
    let input = input_as_string();
    let instructions = parse(&input[..]);
    println!("Part 1: {}", doit(&instructions, 1));
    println!("Part 2: {}", doit(&instructions, 2));
}
