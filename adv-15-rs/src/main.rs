#![feature(bool_to_option)]
#![feature(destructuring_assignment)]

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::iter::from_fn;
use std::mem::swap;
use std::ops::{Index, IndexMut};

const FILENAME: &'static str = "i1.txt";

struct Grid<T> {
    rows: usize,
    cols: usize,
    backer: Vec<T>
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.backer.as_slice()[self.cols*index..self.cols*(index+1)]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.backer.as_mut_slice()[self.cols*index..self.cols*(index+1)]
    }
}

impl<T> Grid<T> {
    fn copy_map<U>(&self, mut f: impl FnMut(&T) -> U) -> Grid<U> {
        Grid {
            rows: self.rows,
            cols: self.cols,
            backer: self.backer.iter().map(f).collect()
        }
    }

    fn from_iterator(mut it: impl Iterator<Item=impl Iterator<Item=T>>) -> Grid<T> {
        let mut rows = 0;
        let mut cols = 0;
        let mut back = Vec::new();
        for sub_it in it {
            back.extend(sub_it);
            if rows == 0 {
                cols = back.len();
            }
            rows += 1;
        }
        Grid {
            rows,
            cols,
            backer: back
        }
    }

    fn find_adjacent(&self, y: usize, x: usize) -> impl Iterator<Item=(usize, usize)> {
        [
            (y != 0).then(|| (y - 1, x)),
            (x != 0).then(|| (y, x - 1)),
            ((y + 1) < self.rows).then(|| (y + 1, x)),
            ((x + 1) < self.cols).then(|| (y, x + 1))
        ].into_iter().filter_map(|v| v)
    }

    fn find_adjacent_values(&self, y: usize, x: usize) -> impl Iterator<Item=&T> {
        self
            .find_adjacent(y, x)
            .map(|(y, x)| &self[y][x])
    }

    fn with_size(rows: usize, cols: usize, mut f: impl FnMut(usize, usize) -> T) -> Grid<T> {
        Grid {
            rows,
            cols,
            backer: (0..rows).map(|y| {
                (0..cols).map(move |x| (y, x))
            }).flatten().map(|(y, x)| f(y, x)).collect()
        }
    }
}

impl Grid<u8> {
    fn cheapest_path(&self) -> usize {
        let mut score_grid = self.copy_map(|_| usize::MAX);
        let mut update_queue = vec![(self.rows - 1, self.cols - 1)];

        while let Some((y, x)) = update_queue.pop() {
            let old = score_grid[y][x];
            let new = {
                if ((y + 1) == self.rows) && ((x + 1) == self.cols) {
                    0
                } else {
                    score_grid
                        .find_adjacent(y, x)
                        .map(|(y2, x2)| {
                            score_grid[y2][x2]
                                .checked_add(self[y2][x2] as usize)
                                .unwrap_or(usize::MAX)
                        }).min().unwrap()
                }
            };
            if old != new {
                score_grid[y][x] = new;
                update_queue.extend(score_grid.find_adjacent(y, x))
            }
        }

        score_grid[0][0]
    }
}

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open(FILENAME).unwrap();
    let mut buf = BufReader::new(file);
    from_fn(move || {
        let mut s = String::new();
        if buf.read_line(&mut s).unwrap() == 0 {
            None
        } else {
            Some(s)
        }
    })
}

fn read_data() -> Grid<u8> {
    Grid::from_iterator(
        read_lines().map(|s| {
            s.chars()
                .filter(char::is_ascii_digit)
                .map(|c| (c as u8) - ('0' as u8))
                .collect::<Vec<_>>()
                .into_iter()
        })
    )
}

fn inc_by<T: Eq + Hash>(map: &mut HashMap<T, usize>, k: T, v: usize) {
    match map.entry(k) {
        Entry::Occupied(mut o) => {
            *o.get_mut() += v;
        }
        Entry::Vacant(e) => {
            e.insert(v);
        }
    }
}

fn main() {
    let risk_grid = read_data();
    let risk_grid_2 = Grid::with_size(
        risk_grid.rows * 5,
        risk_grid.cols * 5,
        |y, x| {
            let by = (y / risk_grid.rows) as u8;
            let bx = (x / risk_grid.cols) as u8;
            let ly = y % risk_grid.rows;
            let lx = x % risk_grid.cols;
            ((by + bx + risk_grid[ly][lx] - 1) % 9) + 1
        });

    println!("1> {}", risk_grid.cheapest_path());
    println!("2> {}", risk_grid_2.cheapest_path());
}
