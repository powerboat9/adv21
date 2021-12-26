#![feature(linked_list_cursors)]
#![feature(int_abs_diff)]
#![feature(map_first_last)]

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;
use std::fmt::{Arguments, Display, Formatter, Write};
use std::fs::{File, read};
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::iter::from_fn;
use std::mem::swap;
use std::ops::Rem;
use std::process::exit;
use std::thread::Builder;

const FILENAME: &'static str = "i1.txt";

fn read_lines() -> impl Iterator<Item=String> {
    let file = File::open(FILENAME).unwrap();
    let mut buf = BufReader::new(file);
    from_fn(move || {
        let mut s = String::new();
        if buf.read_line(&mut s).unwrap() == 0 {
            None
        } else {
            if s.ends_with('\n') {
                s.pop();
            }
            Some(s)
        }
    })
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Space {
    Empty,
    Down,
    Right
}

impl Space {
    fn get_diff(&self) -> (usize, usize) {
        match self {
            Space::Empty => unreachable!(),
            Space::Down => (0, 1),
            Space::Right => (1, 0)
        }
    }
}

fn parse_line<'a>(s: &'a str) -> impl 'a + Iterator<Item=Space> {
    s
        .chars()
        .map(|c| match c {
            '.' => Space::Empty,
            '>' => Space::Right,
            'v' => Space::Down,
            _ => panic!("parse failure")
        })
}

fn read_data() -> Grid {
    let lines = read_lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    Grid {
        data: lines.into_iter()
            .flat_map(|l| parse_line(l.as_str()).collect::<Vec<_>>().into_iter())
            .collect(),
        width,
        height
    }
}

struct Grid {
    data: Vec<Space>,
    width: usize,
    height: usize
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y != 0 {
                f.write_char('\n')?
            }
            for x in 0..self.width {
                f.write_char(match self.data[y * self.width + x] {
                    Space::Empty => '.',
                    Space::Down => 'v',
                    Space::Right => '>'
                })?
            }
        }
        Ok(())
    }
}

impl Grid {
    fn correct_coords(&self, x: usize, y: usize) -> (usize, usize) {
        let y = y % self.height;
        let x = x % self.width;
        (x, y)
    }

    fn is_occupied(&self, x: usize, y: usize) -> bool {
        let (x, y) = self.correct_coords(x, y);
        self.data[y * self.width + x] != Space::Empty
    }

    fn set(&mut self, x: usize, y: usize, s: Space) {
        let (x, y) = self.correct_coords(x, y);
        self.data[y * self.width + x] = s;
    }

    fn tick_space(&mut self, s: Space) -> bool {
        let mut did_move = false;
        let mut new_board = Grid {
            data: vec![Space::Empty; self.data.len()],
            width: self.width,
            height: self.height
        };
        let diff = s.get_diff();
        for y in 0..self.height {
            for x in 0..self.width {
                if (self.data[y * self.width + x] == s) && !self.is_occupied(x + diff.0, y + diff.1) {
                    new_board.set(x + diff.0, y + diff.1, s);
                    did_move = true;
                } else if self.data[y * self.width + x] != Space::Empty {
                    new_board.set(x, y, self.data[y * self.width + x]);
                }
            }
        }
        swap(&mut new_board, self);
        did_move
    }

    fn tick(&mut self) -> bool {
        let r = self.tick_space(Space::Right);
        let d = self.tick_space(Space::Down);
        r || d
    }
}

fn main() {
    let mut data = read_data();

    for i in 1.. {
        //println!("i: {}", i);
        //println!("{}", &data);
        if !data.tick() {
            println!("1> {}", i);
            break;
        }
    }
}
