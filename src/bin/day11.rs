use std::vec::Vec;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;

fn mkgrid(input: &[String]) -> Grid<u8> {
    Grid::from_input_map(input, 0, 0, |c| match c {
        '0'..='9' => c as u8 - b'0',
        _ => panic!(),
    })
}

fn simstep(grid: &mut Grid<u8>) -> usize {
    let xb = grid.x_bounds();
    let yb = grid.y_bounds();
    let mut total_flashes = 0;
    grid.iter_mut().for_each(|v| *v += 1);
    loop {
        let mut flashes = 0;
        for y in yb.start..yb.end {
            for x in xb.start..xb.end {
                if grid.get(x, y) > 9 {
                    grid.set(x, y, 0);
                    flashes += 1;
                    let mut incr = |nx, ny| {
                        if nx >= xb.start && nx < xb.end && ny >= yb.start && ny < yb.end {
                            let v = grid.get(nx, ny);
                            if v != 0 {
                                grid.set(nx, ny, v + 1);
                            }
                        }
                    };
                    incr(x + 1, y);
                    incr(x + 1, y + 1);
                    incr(x + 1, y - 1);
                    incr(x - 1, y);
                    incr(x - 1, y + 1);
                    incr(x - 1, y - 1);
                    incr(x, y + 1);
                    incr(x, y - 1);
                }
            }
        }
        if flashes == 0 { break; }
        total_flashes += flashes;
    }
    total_flashes
}

fn part1(grid: &Grid<u8>) -> usize {
    let mut grid = grid.clone();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += simstep(&mut grid);
    }
    flashes
}

fn part2(grid: &Grid<u8>) -> usize {
    let mut grid = grid.clone();
    let n_octos = grid.iter().count();
    let mut iters = 0;
    loop {
        iters += 1;
        if simstep(&mut grid) >= n_octos {
            break;
        }
    }
    iters
}

fn main() {
    let input: Vec<String> = read_input();
    let grid = mkgrid(&input);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use ya_advent_lib::read::test_input;
    use super::*;

    #[test]
    fn day11_test() {
        let input: Vec<String> = test_input(include_str!("day11.testinput"));
        let grid = mkgrid(&input);
        assert_eq!(part1(&grid), 1656);
        assert_eq!(part2(&grid), 195);
    }
}
