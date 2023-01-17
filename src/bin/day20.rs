use std::vec::Vec;
extern crate advent2021;
use advent2021::read::read_grouped_input;
use advent2021::infinite_grid::InfiniteGrid;

#[derive(Clone, Copy)]
enum Pixel {
    Dark,
    Light,
    Untouched,
}

fn setup(input: &Vec<Vec<String>>) -> (InfiniteGrid<Pixel>, Vec<Pixel>) {
    let mapfunc = |c| match c { '.' => Pixel::Dark, '#' => Pixel::Light, _ => panic!() };
    let enh_map = input[0][0].chars().map(mapfunc).collect();
    let grid = InfiniteGrid::from_input(&input[1], Pixel::Untouched, |c,_,_| Some(mapfunc(c)));
    (grid, enh_map)
}

fn step(grid: &InfiniteGrid<Pixel>, enh_map: &Vec<Pixel>, iter_num: usize) -> InfiniteGrid<Pixel> {
    let xb = grid.x_bounds();
    let yb = grid.y_bounds();
    let unknown_flip = matches!(enh_map[0], Pixel::Light);
    let unknown_val:usize = if unknown_flip && (iter_num & 1 == 1) { 1 } else { 0 };
    let mut newgrid = InfiniteGrid::new(Pixel::Untouched);
    for y in (yb.start - 1)..(yb.end + 1) {
        for x in (xb.start - 1)..(xb.end + 1) {
            let mut index = 0usize;
            for (px, py) in [
                (x-1, y-1), (x, y-1), (x+1, y-1),
                (x-1, y),   (x, y),   (x+1, y),
                (x-1, y+1), (x, y+1), (x+1, y+1),
            ] {
                index <<= 1;
                index |= match grid.get(px, py) {
                    Pixel::Dark => 0,
                    Pixel::Light => 1,
                    Pixel::Untouched => unknown_val,
                };
            }
            newgrid.set(x, y, enh_map[index]);
        }
    }
    newgrid
}

#[allow(dead_code)]
fn printgrid(grid: &InfiniteGrid<Pixel>) {
    grid.print(|cell| match cell {
        Pixel::Light => '#',
        Pixel::Dark => '.',
        Pixel::Untouched => ' ',
    });
}

fn part1(grid: &InfiniteGrid<Pixel>, enh_map: &Vec<Pixel>) -> usize {
    let grid = step(&grid, enh_map, 0);
    let grid = step(&grid, enh_map, 1);
    grid.iter()
        .filter(|(_,c)| matches!(c, Pixel::Light))
        .count()
}

fn part2(grid: &InfiniteGrid<Pixel>, enh_map: &Vec<Pixel>) -> usize {
    let mut grid = grid.clone();
    for n in 0..50 {
        grid = step(&grid, enh_map, n);
    }
    //println!("x: {:?}, y: {:?}", grid.x_bounds(), grid.y_bounds());
    grid.iter()
        .filter(|(_,c)| matches!(c, Pixel::Light))
        .count()
}

fn main() {
    let input: Vec<Vec<String>> = read_grouped_input();
    let (grid, enh_map) = setup(&input);
    println!("Part 1: {}", part1(&grid, &enh_map));
    println!("Part 2: {}", part2(&grid, &enh_map));
}

#[cfg(test)]
mod tests {
    use advent2021::read::grouped_test_input;
    use super::*;

    #[test]
    fn day20_test() {
        let input: Vec<Vec<String>> = grouped_test_input(include_str!("day20.testinput"));
        let (grid, enh_map) = setup(&input);
        assert_eq!(part1(&grid, &enh_map), 35);
        assert_eq!(part2(&grid, &enh_map), 3351);
    }
}
