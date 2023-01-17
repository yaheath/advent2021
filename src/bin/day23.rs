use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::iter;
use std::vec::Vec;
extern crate advent2021;
use advent2021::read::read_input;
use advent2021::grid::Grid;

#[derive(Clone, Copy)]
enum Cell {
    Wall,
    Space,
    Pod(Pod),
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Pod {
    A=0, B=1, C=2, D=3,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Rule {
    Hall,
    Entry,
    Room(Pod),
    Wall,
}

impl Pod {
    fn cost(&self) -> usize {
        match *self {
            Pod::A => 1,
            Pod::B => 10,
            Pod::C => 100,
            Pod::D => 1000,
        }
    }
}

struct Map {
    cells: Grid<Cell>,
    rules: Grid<Rule>,
    rooms: [Vec<Coord>; 4],
    halls: Vec<Coord>,
    idx_to_type: Vec<Pod>,
}

fn make_map(input: &Vec<String>) -> Map {
    let cells = Grid::from_input(&input, Cell::Wall, 0, |c| match c {
        '#' => Cell::Wall,
        '.' => Cell::Space,
        'A' => Cell::Pod(Pod::A),
        'B' => Cell::Pod(Pod::B),
        'C' => Cell::Pod(Pod::C),
        'D' => Cell::Pod(Pod::D),
        _   => Cell::Wall,
    });
    let mut rules: Grid<Rule> = Grid::new(
        cells.x_bounds().start, cells.y_bounds().start,
        cells.x_bounds().end - 1, cells.y_bounds().end - 1,
        Rule::Wall);
    let mut halls: Vec<Coord> = Vec::new();

    let mut room_y = -1i64;
    let mut room_y_top = -1i64;
    let rooms = [Pod::A, Pod::B, Pod::C, Pod::D];
    let mut room_x: [i64; 4] = [0, 0, 0, 0];
    let mut room_idx = 0usize;
    for y in cells.y_bounds().rev() {
        for x in cells.x_bounds() {
            rules.set(x, y,
                match cells.get(x, y) {
                    Cell::Wall => Rule::Wall,
                    Cell::Pod(_) => {
                        if room_y < 0 {
                            room_y = y;
                        }
                        room_y_top = y;
                        let p = rooms[room_idx];
                        room_x[room_idx] = x;
                        room_idx = (room_idx + 1) % 4;
                        Rule::Room(p)
                    },
                    Cell::Space => {
                        if y == room_y_top - 1 && matches!(cells.get(x, y+1), Cell::Pod(_)) {
                            Rule::Entry
                        } else {
                            halls.push((x, y));
                            Rule::Hall
                        }
                    },
                }
            );
        }
    }
    let rooms = [
        (room_y_top..=room_y).map(|y| (room_x[0], y)).collect::<Vec<Coord>>(),
        (room_y_top..=room_y).map(|y| (room_x[1], y)).collect::<Vec<Coord>>(),
        (room_y_top..=room_y).map(|y| (room_x[2], y)).collect::<Vec<Coord>>(),
        (room_y_top..=room_y).map(|y| (room_x[3], y)).collect::<Vec<Coord>>(),
    ];
    let ppt = rooms[0].len();
    let idx_to_type: Vec<Pod> = Vec::from_iter(
        [Pod::A, Pod::B, Pod::C, Pod::D].iter()
        .map(|p| iter::repeat(*p).take(ppt))
        .flatten()
    );
    Map {
        cells,
        rules,
        rooms,
        halls,
        //ppt,
        idx_to_type,
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Grid<Cell>) {
    grid.print(|c| match c {
        Cell::Wall => '#',
        Cell::Space => '.',
        Cell::Pod(Pod::A) => 'A',
        Cell::Pod(Pod::B) => 'B',
        Cell::Pod(Pod::C) => 'C',
        Cell::Pod(Pod::D) => 'D',
    });
}

#[allow(dead_code)]
fn print_rule(grid: &Grid<Rule>) {
    grid.print(|c| match c {
        Rule::Wall => '#',
        Rule::Hall => '.',
        Rule::Entry => '-',
        Rule::Room(Pod::A) => 'A',
        Rule::Room(Pod::B) => 'B',
        Rule::Room(Pod::C) => 'C',
        Rule::Room(Pod::D) => 'D',
    });
}

type Coord = (i64, i64);

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pods: Vec<Coord>,
}

impl State {
    fn pods_as_array(&self) -> [Coord; 16] {
        let mut out = [(0, 0); 16];
        self.pods.iter().enumerate().for_each(|(idx, &c)| {
            out[idx] = c;
        });
        out
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| other.pods.cmp(&self.pods))
    }
}
// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_initial_state(map: &Map) -> State {
    let mut a_pods: Vec<Coord> = Vec::new();
    let mut b_pods: Vec<Coord> = Vec::new();
    let mut c_pods: Vec<Coord> = Vec::new();
    let mut d_pods: Vec<Coord> = Vec::new();
    map.cells.for_each(|c, x, y| match c {
        Cell::Pod(Pod::A) => {
            a_pods.push((x, y));
        },
        Cell::Pod(Pod::B) => {
            b_pods.push((x, y));
        },
        Cell::Pod(Pod::C) => {
            c_pods.push((x, y));
        },
        Cell::Pod(Pod::D) => {
            d_pods.push((x, y));
        },
        _ => {},
    });
    let pods = a_pods.iter()
        .chain(b_pods.iter())
        .chain(c_pods.iter())
        .chain(d_pods.iter())
        .cloned()
        .collect();
    State {
        cost: 0,
        pods
    }
}

fn is_final(coords: &Vec<Coord>, map: &Map) -> bool {
    map.idx_to_type.iter()
        .enumerate()
        .filter(|(idx, p)| map.rules.get(coords[*idx].0, coords[*idx].1) == Rule::Room(**p))
        .count()
    == map.idx_to_type.len()
}

enum Move {
    ToHall,
    ToRoom,
    Stay,
}

struct StateExtra {
    cost: usize,
    pods: Vec<Coord>,
    positions: HashMap<Coord, usize>,
}

impl StateExtra {
    fn new(state: &State) -> Self {
        let positions: HashMap<Coord, usize> = HashMap::from_iter(
            state.pods.iter()
            .enumerate()
            .map(|(idx,c)| (*c, idx))
        );
        Self {
            cost: state.cost,
            pods: state.pods.clone(),
            positions,
        }
    }
}

fn path_to(idx: usize, dest: Coord, state: &StateExtra, map: &Map) -> Option<State> {
    let mut queue:Vec<(Coord,usize)> = Vec::new();
    let mut traversed:HashSet<Coord> = HashSet::new();
    queue.push((state.pods[idx], 0));
    traversed.insert(state.pods[idx]);
    while let Some((pos, steps)) = queue.pop() {
        if pos == dest {
            let mut newpods = state.pods.clone();
            newpods[idx] = pos;
            let cost = steps * map.idx_to_type[idx].cost() + state.cost;
            return Some(State { cost, pods: newpods });
        }
        let at_entry = map.rules.get(pos.0, pos.1) == Rule::Entry;
        for newpos in [
            (pos.0 - 1, pos.1), (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1), (pos.0, pos.1 + 1),
        ] {
            if state.positions.contains_key(&newpos) { continue; }
            match map.rules.get(newpos.0, newpos.1) {
                Rule::Wall => { continue; },
                Rule::Room(r) => {
                    if r != map.idx_to_type[idx] && at_entry { continue; }
                },
                _ => {},
            }
            if !traversed.contains(&newpos) {
                queue.push((newpos, steps + 1));
                traversed.insert(newpos);
            }
        }
    }
    None
}

fn get_next_moves<'a>(state: &'a StateExtra, map: &'a Map) -> impl Iterator<Item=State> + 'a {
    map.idx_to_type.iter()
        .enumerate()
        .map(move |(idx, p)| {
            let (x, y) = state.pods[idx];
            let moveto = match map.rules.get(x, y) {
                Rule::Room(rm) => {
                    if rm == *p {
                        // already in dest room. Stay put unless
                        // there is an incorrect Pod here
                        if map.rules.get(x, y+1) != Rule::Wall &&
                            state.positions.contains_key(&(x, y+1)) &&
                            map.idx_to_type[state.positions[&(x, y+1)]] != *p
                        {
                            Move::ToHall
                        } else {
                            Move::Stay
                        }
                    }
                    else {
                        Move::ToHall
                    }
                },
                Rule::Hall => {
                    if state.positions.iter()
                        .filter(|(c, _)| map.rules.get(c.0, c.1) == Rule::Room(*p))
                        .filter(|(_, idx)| map.idx_to_type[**idx] != *p)
                        .count() > 0 {
                        // someone is in the room that doesn't belong
                        Move::Stay
                    } else {
                        Move::ToRoom
                    }
                },
                _ => panic!(),
            };
            let dests: Vec<Coord> = match moveto {
                Move::Stay => vec![],
                Move::ToRoom => {
                    let mut v: Vec<Coord> = Vec::new();
                    if let Some(loc) = map.rooms[*p as usize]
                            .iter().rev()
                            .find(|&pos| !state.positions.contains_key(pos)) {
                        v.push(*loc);
                    }
                    v
                },
                Move::ToHall => map.halls.clone(),
            };
            dests.iter()
                .map(|d| path_to(idx, *d, &state, map))
                .filter(|s| s.is_some())
                .map(|s| s.unwrap())
                .collect::<Vec<State>>()
        })
        .flatten()
}

fn search(map: &Map) -> usize {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashMap<[Coord; 16], usize> = HashMap::new();
    let initial_state = get_initial_state(map);
    visited.insert(initial_state.pods_as_array(), 0);
    heap.push(initial_state);
    while let Some(state) = heap.pop() {
        if is_final(&state.pods, map) {
            return state.cost;
        }
        let stateextra = StateExtra::new(&state);
        for next in get_next_moves(&stateextra, map) {
            let key = next.pods_as_array();
            if !visited.contains_key(&key) || visited[&key] > next.cost {
                visited.insert(key, next.cost);
                heap.push(next);
            }
        }
    }
    panic!("no solution found");
}

fn part1(input: &Vec<String>) -> usize {
    let map = make_map(input);
    /*
    print_grid(&map.cells);
    println!("");
    print_rule(&map.rules);
    */
    search(&map)
}

fn part2(input: &Vec<String>) -> usize {
    let map = make_map(input);
    search(&map)
}

fn modify_for_part2(input: &mut Vec<String>) {
    input.insert(3, "  #D#C#B#A#".into());
    input.insert(4, "  #D#B#A#C#".into());
}

fn main() {
    let mut input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    modify_for_part2(&mut input);
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use advent2021::read::test_input;
    use super::*;

    #[test]
    fn day23_test() {
        let mut input: Vec<String> = test_input(include_str!("day23.testinput"));
        assert_eq!(part1(&input), 12521);
        modify_for_part2(&mut input);
        assert_eq!(part2(&input), 44169);
    }
}
