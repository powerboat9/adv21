#![feature(map_first_last)]
#![feature(int_abs_diff)]
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{from_fn, FusedIterator};
use std::mem::swap;
use std::ops::{Add, AddAssign, Div, Sub};

use regex::Regex;

const FILENAME: &'static str = "i1.txt";
lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)").unwrap();
}

fn read_lines() -> impl Iterator<Item=String> {
    let file = File::open(FILENAME).unwrap();
    let read = BufReader::new(file);
    read.lines().map(|v| v.unwrap())
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum LineEventType {
    Start,
    End,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct LineEvent {
    line: Line,
    t: LineEventType,
}

fn read_data() -> impl Iterator<Item=Line> {
    read_lines().map(|s| {
        let m = LINE_RE.captures(s.as_str()).unwrap();
        let mut l = Line {
            start: Vec2i {
                x: m.get(1).unwrap().as_str().parse().unwrap(),
                y: m.get(2).unwrap().as_str().parse().unwrap(),
            },
            end: Vec2i {
                x: m.get(3).unwrap().as_str().parse().unwrap(),
                y: m.get(4).unwrap().as_str().parse().unwrap(),
            },
        };
        if l.start > l.end {
            swap(&mut l.start, &mut l.end);
        }
        l
    })
}

fn simple_line_to_events(line: Line) -> [LineEvent; 2] {
    [
        LineEvent {
            line,
            t: LineEventType::Start,
        },
        LineEvent {
            line,
            t: LineEventType::End,
        }
    ]
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Direction {
    Horizontal,
    Vertical
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct SimpleLine {
    p: Vec2i,
    d: Direction,
    len: u32
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum LineEndType {
    Start,
    End
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct SimpleLineEvent {
    p: Vec2i,
    d: Direction,
    t: LineEndType
}

impl Into<[SimpleLineEvent; 2]> for SimpleLine {
    fn into(self) -> [SimpleLineEvent; 2] {
        [
            SimpleLineEvent {
                p: self.p,
                d: self.d,
                t: LineEndType::Start
            },
            SimpleLineEvent {
                p: match self.d {
                    Direction::Horizontal => self.p + Vec2i::new(self.len.try_into().unwrap(), 0),
                    Direction::Vertical => self.p + Vec2i::new(0, self.len.try_into().unwrap())
                },
                d: self.d,
                t: LineEndType::End
            }
        ]
    }
}

/*
// assumes sorted input
// does not sort output
fn combine_overlap(lines: impl Iterator<Item=SimpleLineEvent> + FusedIterator, intersect: HashSet<Point>) -> impl Iterator<Item=SimpleLineEvent> {
    let mut cur_hz = None;
    let mut hz_cnt: u32 = 0;
    let mut cur_vt = None;
    let mut vt_cnt: u32 = 0;
    from_fn(move || {
        while let Some(e) = lines.next() {
            match (e.t, e.d) {
                (LineEndType::Start, Direction::Horizontal) => {
                    if ()
                }
            }
        }
        cur_hz.or(cur_vt);
    })
}
*/

fn calc_p1(input: &BTreeSet<Line>) {
    let mut cross_map = HashMap::new();
    for p in input
        .iter().copied()
        .filter(|v| (v.start.x == v.end.x) || (v.start.y == v.end.y))
        .flat_map(|v| v.get_points())
    {
        match cross_map.entry(p) {
            Entry::Occupied(mut o) => {
                *o.get_mut() = true;
            },
            Entry::Vacant(v) => {
                v.insert(false);
            }
        }
    }
    let ans = cross_map.drain().filter_map(|v| {
        if v.1 {
            Some(v.0)
        } else {
            None
        }
    }).count();
    println!("1> {}", ans);
}

fn calc_p2(input: &BTreeSet<Line>) {
    let mut cross_map = HashMap::new();
    for p in input
        .iter().copied()
        .flat_map(|v| v.get_points())
    {
        match cross_map.entry(p) {
            Entry::Occupied(mut o) => {
                *o.get_mut() = true;
            },
            Entry::Vacant(v) => {
                v.insert(false);
            }
        }
    }
    let ans = cross_map.drain().filter_map(|v| {
        if v.1 {
            Some(v.0)
        } else {
            None
        }
    }).count();
    println!("2> {}", ans);
}

fn main() {
    let input: BTreeSet<Line> = read_data().collect();
    calc_p1(&input);
    calc_p2(&input);
}
