#![feature(map_first_last)]
#![feature(int_abs_diff)]
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::iter::{from_fn, FusedIterator};
use std::mem::swap;
use std::ops::{Add, AddAssign, Div, Sub};

use regex::Regex;

const FILENAME: &'static str = "i1.txt";
lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"\d+").unwrap();
}

fn read_data() -> Vec<i32> {
    let mut file = File::open(FILENAME).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    LINE_RE.find_iter(data.as_str()).map(|v| v.as_str().parse().unwrap()).collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Vec2i {
    x: i32,
    y: i32
}

impl Add for Vec2i {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign for Vec2i {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}

impl Sub for Vec2i {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2i {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Vec2i {
    fn unit_diff(&self) -> Self {
        let d = gcd(self.x.unsigned_abs(), self.y.unsigned_abs());
        Vec2i {
            x: self.x / (d as i32),
            y: self.y / (d as i32)
        }
    }

    fn new(x: i32, y: i32) -> Self {
        Vec2i {
            x,
            y
        }
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    if a < b {
        swap(&mut a, &mut b)
    }
    if a == 0 {
        return 1;
    }
    while b != 0 {
        a = a % b;
        swap(&mut a, &mut b);
    }
    a
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Line {
    start: Vec2i,
    end: Vec2i,
}

impl Line {
    fn get_points(self) -> impl Iterator<Item = Vec2i> {
        let u_delta = (self.end - self.start).unit_diff();
        let mut pos = self.start;
        from_fn(move || {
            if pos > self.end {
                None
            } else {
                let r = pos;
                pos += u_delta;
                Some(r)
            }
        })
    }
}

impl PartialOrd<Self> for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

fn p1(data: &Vec<i32>) {
    let pos = data[data.len() / 2];
    println!("1> {}", data.iter().copied().map(|v| (pos - v).abs()).sum::<i32>());
}

fn p2(data: &Vec<i32>) {
    let res = ((*data.first().unwrap())..=(*data.last().unwrap())).map(|i| {
        data.iter().map(|n| {
            let d = (*n - i).abs();
            (d * (d + 1)) / 2
        }).sum::<i32>()
    }).min().unwrap();
    println!("2> {}", res)
}

fn main() {
    let mut data = read_data();
    data.sort();
    p1(&data);
    p2(&data);
    //println!("2> {}", sum_data(&data));
}
