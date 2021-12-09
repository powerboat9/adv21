#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{from_fn};
use std::ops::{BitAnd, BitOr, BitOrAssign, BitXor};

use regex::Regex;

const A: LetterSet = LetterSet::new(1);
const B: LetterSet = LetterSet::new(2);
const C: LetterSet = LetterSet::new(4);
const D: LetterSet = LetterSet::new(8);
const E: LetterSet = LetterSet::new(16);
const F: LetterSet = LetterSet::new(32);
const G: LetterSet = LetterSet::new(64);

const FILENAME: &'static str = "i1.txt";
lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"(\S+)").unwrap();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct LetterSet {
    inner: u32
}

impl LetterSet {
    const fn new(num: u32) -> Self {
        LetterSet {
            inner: num
        }
    }

    fn letter_cnt(&self) -> u32 {
        self.inner.count_ones()
    }
}

impl BitOr for LetterSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        LetterSet {
            inner: self.inner | rhs.inner
        }
    }
}

impl BitOrAssign for LetterSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner |= rhs.inner;
    }
}

impl BitAnd for LetterSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        LetterSet {
            inner: self.inner & rhs.inner
        }
    }
}

impl BitXor for LetterSet {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        LetterSet {
            inner: self.inner ^ rhs.inner
        }
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

fn read_data() -> impl Iterator<Item = Vec<LetterSet>> {
    read_lines().map(|l| {
        LINE_RE.captures_iter(l.as_str())
            .map(|c| c.get(1).unwrap().as_str())
            .filter_map(|e| {
                if e == "|" {
                    None
                } else {
                    Some(e.chars().fold(LetterSet::new(0), |acc, c| {
                        match c {
                            'a' => acc | A,
                            'b' => acc | B,
                            'c' => acc | C,
                            'd' => acc | D,
                            'e' => acc | E,
                            'f' => acc | F,
                            'g' => acc | G,
                            _ => panic!()
                        }
                    }))
                }
            })
            .collect()
    })
}

fn get_true_digits(digits: &[LetterSet]) -> [LetterSet; 10] {
    let mut ret = [LetterSet::new(0); 10];
    // find 1, 4, 7, 8
    ret[8] = LetterSet::new(127);
    for d in digits {
        match d.letter_cnt() {
            2 => ret[1] = *d,
            4 => ret[4] = *d,
            3 => ret[7] = *d,
            _ => {}
        }
    }
    // find 3 and 6 using 1
    for d in digits {
        if *d == LetterSet::new(127) {
        } else if (*d | ret[1]) == LetterSet::new(127) {
            ret[6] = *d;
        } else if (*d ^ ret[1]).letter_cnt() == 3 {
            ret[3] = *d;
        }
    }
    // find 9 using 3 and 4
    ret[9] = ret[3] | ret[4];
    // find 2, 5, 0 using xor from here
    ret[2] = ret[1] ^ ret[3] ^ ret[6] ^ ret[9];
    ret[5] = ret[1] ^ ret[2] ^ ret[3] ^ ret[8];
    ret[0] = ret[2] ^ ret[4] ^ ret[6] ^ ret[8];
    ret
}

fn p1(data: &Vec<Vec<LetterSet>>) {
    let r = data.iter()
        .map(|v| v.iter().skip(10))
        .flatten().filter_map(|v| {
        match v.letter_cnt() {
            2 => Some(()),
            4 => Some(()),
            3 => Some(()),
            7 => Some(()),
            _ => None
        }
    }).count();
    println!("1> {}", r);
}

fn p2(data: &Vec<Vec<LetterSet>>) {
    let sum = data.iter().map(|entry| {
        let true_dig = get_true_digits(&entry.as_slice()[0..10]);
        entry.as_slice()[10..14].iter().map(|d| {
            true_dig.iter()
                .enumerate()
                .find_map(|v| {
                    if *d == *v.1 {
                        Some(v.0)
                    } else {
                        None
                    }
                }).unwrap()
        }).fold(0, |acc, v| acc * 10 + (v as u64))
    }).sum::<u64>();
    println!("2> {}", sum);
}

fn main() {
    let data = read_data().collect::<Vec<_>>();
    p1(&data);
    p2(&data);
}
