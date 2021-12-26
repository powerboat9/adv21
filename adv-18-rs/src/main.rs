#![feature(bool_to_option)]
#![feature(destructuring_assignment)]

use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{from_fn, Sum};
use std::ops::{Add, Rem};

const FILENAME: &'static str = "i1.txt";

fn read_lines() -> impl Iterator<Item=String> {
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

#[derive(Clone)]
enum SnailNum {
    Lit(u32),
    Pair(Box<SnailNum>, Box<SnailNum>)
}

impl SnailNum {
    fn leftmost(&mut self) -> &mut u32 {
        match self {
            SnailNum::Lit(n) => n,
            SnailNum::Pair(b, _) => b.leftmost()
        }
    }

    fn rightmost(&mut self) -> &mut u32 {
        match self {
            SnailNum::Lit(n) => n,
            SnailNum::Pair(_, b) => b.rightmost()
        }
    }

    fn assume_lit(&self) -> u32 {
        match self {
            SnailNum::Lit(n) => *n,
            _ => unreachable!()
        }
    }

    fn attempt_explode(&mut self, l: Option<&mut u32>, r: Option<&mut u32>, depth: usize) -> bool {
        match self {
            SnailNum::Lit(n) => false,
            SnailNum::Pair(a, b) => {
                if depth == 0 {
                    if let Some(l) = l {
                        *l += a.assume_lit()
                    }
                    if let Some(r) = r {
                        *r += b.assume_lit()
                    }
                    *self = SnailNum::Lit(0);
                    true
                } else {
                    a.attempt_explode(l, Some(b.leftmost()), depth - 1)
                        || b.attempt_explode(Some(a.rightmost()), r, depth - 1)
                }
            }
        }
    }

    fn attempt_split(&mut self) -> bool {
        match self {
            SnailNum::Lit(n) if *n >= 10 => {
                let a = *n / 2;
                let b = *n - a;
                let a = Box::new(SnailNum::Lit(a));
                let b = Box::new(SnailNum::Lit(b));
                *self = SnailNum::Pair(a, b);
                true
            }
            SnailNum::Pair(a, b) => {
                a.attempt_split() || b.attempt_split()
            }
            _ => false
        }
    }

    fn fully_explode(&mut self) -> bool {
        if !self.attempt_explode(None, None, 4) {
            return false
        }
        while self.attempt_explode(None, None, 4) {}
        true
    }

    fn fully_split(&mut self) -> bool {
        if !self.attempt_split() {
            return false
        }
        while self.attempt_split() {}
        true
    }

    fn normalize(&mut self) {
        let mut dirty = true;
        while dirty {
            dirty = false;
            while self.attempt_explode(None, None, 4) {
                dirty = true;
            }
            dirty = self.attempt_split()
        }
    }

    fn mag(&self) -> u32 {
        match self {
            SnailNum::Lit(n) => *n,
            SnailNum::Pair(a, b) => {
                3 * a.mag() + 2 * b.mag()
            }
        }
    }
}

impl Add for SnailNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut r = SnailNum::Pair(Box::new(self), Box::new(rhs));
        r.normalize();
        r
    }
}

impl Display for SnailNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailNum::Lit(n) => f.write_fmt(format_args!("{}", n)),
            SnailNum::Pair(a, b) => f.write_fmt(format_args!("[{},{}]", a, b))
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum SnailToken {
    Open,
    Num(u32)
}

fn tokenise<'a>(mut s: &'a str) -> impl 'a + Iterator<Item=SnailToken> {
    from_fn(move || {
        loop {
            if s.len() == 0 {
                return None
            } else if s.starts_with('[') {
                s = &s[1..];
                return Some(SnailToken::Open)
            } else if s.starts_with([',', ']', '\n']) {
                s = &s[1..];
            } else {
                let pos = s.char_indices()
                    .take_while(|&(n, c)| c.is_ascii_digit())
                    .last().unwrap()
                    .0;
                let p = &s[..=pos];
                s = &s[pos + 1..];
                return Some(SnailToken::Num(p.parse().unwrap()));
            }
        }
    })
}

fn parse_single(it: &mut impl Iterator<Item=SnailToken>) -> SnailNum {
    match it.next().unwrap() {
        SnailToken::Open => {
            let a = parse_single(it);
            let b = parse_single(it);
            SnailNum::Pair(Box::new(a), Box::new(b))
        }
        SnailToken::Num(n) => SnailNum::Lit(n)
    }
}

fn read_data() -> impl Iterator<Item=SnailNum> {
    read_lines().map(|line| parse_single(&mut tokenise(line.as_str())))
}

fn main() {
    let mut data = read_data().collect::<Vec<_>>();

    let mut acc = data[0].clone();
    for i in 1..data.len() {
        acc = acc + data[i].clone()
    }
    println!("1> {}", acc.mag());

    let mut best = 0;
    for i in 0..data.len() {
        for j in 0..data.len() {
            if i != j {
                let v = (data[i].clone() + data[j].clone()).mag();
                if best < v {
                    best = v
                }
            }
        }
    }
    println!("2> {}", best);
}
