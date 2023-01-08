use std::cmp::max;
use std::io::Write;
use std::ops::Range;
use std::slice::{Iter, IterMut};
use std::vec::Vec;

pub struct Grid<T: Copy> {
    min_x: i64,
    min_y: i64,
    x_size: usize,
    y_size: usize,
    data: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn new(min_x:i64, min_y:i64, max_x:i64, max_y:i64, initial_val: T) -> Self {
        let x_size = (max_x - min_x + 1) as usize;
        let y_size = (max_y - min_y + 1) as usize;
        let mut g = Self {
            min_x: min_x,
            min_y: min_y,
            x_size: x_size,
            y_size: y_size,
            data: Vec::with_capacity(x_size * y_size),
        };
        for _ in 0..x_size * y_size {
            g.data.push(initial_val);
        }
        g
    }

    pub fn from_input<F>(input: &Vec<String>, default_val: T, padding: i64, mapfunc: F) -> Self
            where F: Fn(char) -> T {
        let width = input.iter().map(|s| s.len()).fold(0, |maxw, w| max(w, maxw)) as i64;
        let height = input.len() as i64;
        let mut y = 0i64;
        let mut inst = Self::new(-padding, -padding, width-1+padding, height-1+padding, default_val);
        for line in input.iter() {
            for (ux, c) in line.chars().enumerate() {
                let x = ux as i64;
                inst.set(x, y, mapfunc(c));
            }
            y += 1;
        }
        inst
    }

    pub fn clone(&self) -> Self {
        Grid {
            min_x: self.min_x,
            min_y: self.min_y,
            x_size: self.x_size,
            y_size: self.y_size,
            data: self.data.clone(),
        }
    }

    pub fn clone_without_data(&self, initial_val: T) -> Self {
        Grid {
            min_x: self.min_x,
            min_y: self.min_y,
            x_size: self.x_size,
            y_size: self.y_size,
            data: vec![initial_val; self.data.len()],
        }
    }

    pub fn get(&self, x:i64, y:i64) -> T {
        assert!(x >= self.min_x && x < self.min_x + self.x_size as i64);
        assert!(y >= self.min_y && y < self.min_y + self.y_size as i64);
        let ux:usize = (x - self.min_x) as usize;
        let uy:usize = (y - self.min_y) as usize;
        let idx = uy * self.x_size + ux;
        self.data[idx]
    }

    pub fn get_or_default(&self, x:i64, y:i64, default: T) -> T {
        if x >= self.min_x && x < self.min_x + self.x_size as i64
          && y >= self.min_y && y < self.min_y + self.y_size as i64 {
            let ux:usize = (x - self.min_x) as usize;
            let uy:usize = (y - self.min_y) as usize;
            let idx = uy * self.x_size + ux;
            self.data[idx]
        } else {
            default
        }
    }

    pub fn set(&mut self, x:i64, y:i64, val:T) {
        assert!(x >= self.min_x && x <= self.min_x + self.x_size as i64);
        assert!(y >= self.min_y && y <= self.min_y + self.y_size as i64);
        let ux:usize = (x - self.min_x) as usize;
        let uy:usize = (y - self.min_y) as usize;
        let idx = uy * self.x_size + ux;
        self.data[idx] = val;
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }


    pub fn x_bounds(&self) -> Range<i64> {
        self.min_x .. self.min_x + self.x_size as i64
    }

    pub fn y_bounds(&self) -> Range<i64> {
        self.min_y .. self.min_y + self.y_size as i64
    }

    pub fn dump_to_file<F>(&self, file: &mut dyn Write, formatter: F)
            where F: Fn(T) -> char {
        for y in self.min_y .. self.min_y + self.y_size as i64 {
            for x in self.min_x .. self.min_x + self.x_size as i64 {
                write!(file, "{}", formatter(self.get(x, y))).unwrap();
            }
            writeln!(file, "").unwrap();
        }
    }

    pub fn print<F>(&self, formatter: F)
            where F: Fn(T) -> char {
        for y in self.min_y .. self.min_y + self.y_size as i64 {
            for x in self.min_x .. self.min_x + self.x_size as i64 {
                print!("{}", formatter(self.get(x, y)));
            }
            println!("");
        }
    }

    pub fn find<F>(&self, predicate: F) -> Option<(i64, i64)>
            where F: Fn(T, i64, i64) -> bool {
        for y in self.min_y .. self.min_y + self.y_size as i64 {
            for x in self.min_x .. self.min_x + self.x_size as i64 {
                if predicate(self.get(x, y), x, y) {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn for_each<F>(&self, callback: F)
            where F: Fn(T, i64, i64) {
        for y in self.min_y .. self.min_y + self.y_size as i64 {
            for x in self.min_x .. self.min_x + self.x_size as i64 {
                callback(self.get(x, y), x, y);
            }
        }
    }
}
