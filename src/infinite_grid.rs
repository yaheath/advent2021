use std::cmp::{min,max};
use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::io::Write;
use std::ops::Range;

pub struct InfiniteGrid<T: Copy> {
    default: T,
    data: HashMap<(i64,i64),T>,
    x_range: Range<i64>,
    y_range: Range<i64>,
    flip_y: bool,
}

impl<T: Copy> InfiniteGrid<T> {
    pub fn new(default_val: T) -> Self {
        Self {
            default: default_val,
            data: HashMap::new(),
            x_range: Range { start: 0, end: 0 },
            y_range: Range { start: 0, end: 0 },
            flip_y: false,
        }
    }

    pub fn from_input<F>(input: &Vec<String>, default_val: T, mapfunc: F) -> Self
            where F: Fn(char, i64, i64) -> Option<T> {
        let mut y = 0i64;
        let mut inst = Self::new(default_val);
        for line in input.iter() {
            for (ux, c) in line.chars().enumerate() {
                let x = ux as i64;
                if let Some(val) = mapfunc(c, x, y) {
                    inst.set(x, y, val);
                };
            }
            y += 1;
        }
        inst
    }

    pub fn iter(&self) -> Iter<(i64,i64),T> {
        self.data.iter()
    }

    pub fn clone(&self) -> Self {
        Self {
            default: self.default,
            data: self.data.clone(),
            x_range: self.x_range.clone(),
            y_range: self.y_range.clone(),
            flip_y: self.flip_y,
        }
    }

    pub fn get(&self, x:i64, y:i64) -> T {
        if let Some(cell) = self.data.get(&(x, y)) {
            *cell
        }
        else {
            self.default
        }
    }

    pub fn set(&mut self, x:i64, y:i64, val:T) {
        self.data.insert((x, y), val);
        if self.x_range.is_empty() {
            self.x_range.start = x;
            self.x_range.end = x + 1;
        }
        else if x < self.x_range.start {
            self.x_range.start = x;
        }
        else if x >= self.x_range.end {
            self.x_range.end = x + 1;
        }
        if self.y_range.is_empty() {
            self.y_range.start = y;
            self.y_range.end = y + 1;
        }
        else if y < self.y_range.start {
            self.y_range.start = y;
        }
        else if y >= self.y_range.end {
            self.y_range.end = y + 1;
        }
    }

    // Untested
    pub fn purge(&mut self, min_x: Option<i64>, min_y: Option<i64>, max_x: Option<i64>, max_y: Option<i64>) {
        let mut r: Vec<(i64,i64)> = Vec::new();
        let mut new_x_start: i64 = 0;
        let mut new_y_start: i64 = 0;
        let mut new_x_end: i64 = 0;
        let mut new_y_end: i64 = 0;
        for ((x, y), _) in self.data.iter() {
            match min_x {
                Some(val) => { if x < &val { r.push((*x, *y)); continue; } },
                None => {},
            }
            match min_y {
                Some(val) => { if y < &val { r.push((*x, *y)); continue; } },
                None => {},
            }
            match max_x {
                Some(val) => { if x > &val { r.push((*x, *y)); continue; } },
                None => {},
            }
            match max_y {
                Some(val) => { if y > &val { r.push((*x, *y)); continue; } },
                None => {},
            }
            if new_x_start == new_x_end {
                new_x_start = *x;
                new_y_start = *y;
                new_x_end = *x + 1;
                new_y_end = *y + 1;
            } else {
                new_x_start = min(*x, new_x_start);
                new_y_start = min(*y, new_y_start);
                new_x_end = max(*x + 1, new_x_end);
                new_y_end = max(*y + 1, new_y_end);
            }
        }
        for c in r {
            self.data.remove(&c);
        }
        self.x_range.start = new_x_start;
        self.y_range.start = new_y_start;
        self.x_range.end = new_x_end;
        self.y_range.end = new_y_end;
    }

    pub fn x_bounds(&self) -> Range<i64> {
        self.x_range.clone()
    }

    pub fn y_bounds(&self) -> Range<i64> {
        self.y_range.clone()
    }

    pub fn flip_y(&mut self, val: bool) {
        self.flip_y = val;
    }

    pub fn dump_to_file<F>(&self, file: &mut dyn Write, formatter: F)
            where F: Fn(T) -> char {
        for y in self.y_range.clone() {
            for x in self.x_range.clone() {
                write!(file, "{}", formatter(self.get(x, y))).unwrap();
            }
            writeln!(file, "").unwrap();
        }
    }

    pub fn print<F>(&self, formatter: F)
            where F: Fn(T) -> char {
        if self.flip_y {
            for y in self.y_range.clone().rev() {
                for x in self.x_range.clone() {
                    print!("{}", formatter(self.get(x, y)));
                }
                println!("");
            }
        } else {
            for y in self.y_range.clone() {
                for x in self.x_range.clone() {
                    print!("{}", formatter(self.get(x, y)));
                }
                println!("");
            }
        }
    }
}
