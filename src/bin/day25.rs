use std::mem;
use std::vec::Vec;
use advent_lib::grid::Grid;
use advent_lib::read::read_input;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Cell {
    Left,
    Down,
    Empty
}

fn mkgrid(input: &Vec<String>) -> Grid<Cell> {
    Grid::from_input(input, Cell::Empty, 0, |c| match c {
        'v' => Cell::Down,
        '>' => Cell::Left,
        '.' => Cell::Empty,
        _ => panic!(),
    })
}

#[allow(dead_code)]
fn printgrid(grid: &Grid<Cell>) {
    grid.print(|c| {
        match c {
            Cell::Down => 'v',
            Cell::Left => '>',
            Cell::Empty => '.',
        }
    });
}

fn step(grid: &mut Grid<Cell>) -> bool {
    let width = grid.x_bounds().end;
    let height = grid.y_bounds().end;
    let mut newgrid = grid.clone_without_data(Cell::Empty);
    let mut moved = false;
    grid.for_each(|c, x, y| {
        match c {
            Cell::Left =>
                if grid.get((x+1)%width, y) == Cell::Empty {
                    moved = true;
                    newgrid.set((x+1)%width, y, Cell::Left);
                } else {
                    newgrid.set(x, y, Cell::Left);
                },
            Cell::Down => { newgrid.set(x, y, Cell::Down); },
            _ => {},
        }
    });
    let _ = mem::replace(grid, newgrid);
    let mut newgrid = grid.clone_without_data(Cell::Empty);
    grid.for_each(|c, x, y| {
        match c {
            Cell::Down =>
                if grid.get(x, (y+1)%height) == Cell::Empty {
                    moved = true;
                    newgrid.set(x, (y+1)%height, Cell::Down);
                } else {
                    newgrid.set(x, y, Cell::Down);
                },
            Cell::Left => { newgrid.set(x, y, Cell::Left); },
            _ => {},
        }
    });
    let _ = mem::replace(grid, newgrid);
    moved
}

fn part1(start_grid: &Grid<Cell>) -> usize {
    let mut grid = start_grid.clone();
    for i in 1.. {
        if !step(&mut grid) {
            return i;
        }
        /*
        printgrid(&grid);
        println!("");
        */
    }
    panic!();
}

fn main() {
    let input: Vec<String> = read_input();
    let grid = mkgrid(&input);

    println!("Part 1: {}", part1(&grid));
}

#[cfg(test)]
mod tests {
    use advent_lib::read::test_input;
    use super::*;

    #[test]
    fn day25_test() {
        let input: Vec<String> = test_input(include_str!("day25.testinput"));
        let grid = mkgrid(&input);
        assert_eq!(part1(&grid), 58);
    }
}
