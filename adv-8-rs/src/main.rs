#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{from_fn};
use std::ops::{BitAnd, BitOr, BitOrAssign};

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
    const fn empty() -> Self {
        LetterSet::new(0)
    }

    const fn new(num: u32) -> Self {
        LetterSet {
            inner: num
        }
    }

    // only works properly on single letter sets
    fn next(&self) -> Option<Self> {
        let r = (self.inner << 1) & 127;
        if r != 0 {
            Some(LetterSet::new(r))
        } else {
            None
        }
    }

    fn letter_cnt(&self) -> u32 {
        self.inner.count_ones()
    }

    fn is_empty(&self) -> bool {
        self.inner == 0
    }

    fn intersect(&self, oth: &Self) -> Self {
        *self & *oth
    }

    fn could_map_to(&self, oth: &Self, part_map: &[LetterSet]) -> bool {
        let mut self_n = self.inner;
        let mut mask = LetterSet::empty();
        let mut anti_mask = LetterSet::empty();
        for l in part_map.iter().copied() {
            if (self_n & 1) == 0 {
                anti_mask |= l
            } else {
                mask |= l
            }
            self_n >>= 1;
        }
        oth.intersect(&anti_mask).is_empty() && (oth.intersect(&mask) == mask)
    }

    fn map(&self, map: &[LetterSet; 7]) -> LetterSet {
        let mut ret = LetterSet::empty();
        let mut self_n = self.inner;
        for ent in map.iter().copied() {
            if (self_n & 1) != 0 {
                ret |= ent;
            }
            self_n >>= 1;
        }
        ret
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

lazy_static! {
    static ref LETTER_TABLE: [LetterSet; 10] = [
        A | B | C | E | F | G,
        C | F,
        A | C | D | E | G,
        A | C | D | F | G,
        B | C | D | F,
        A | B | D | F | G,
        A | B | D | E | F | G,
        A | C | F,
        A | B | C | D | E | F | G,
        A | B | C | D | F | G
    ];
}

fn could_match_digit(lt: LetterSet, part_map: &[LetterSet]) -> bool {
    get_possible_match_digit(lt, part_map).is_some()
}

fn get_possible_match_digit(lt: LetterSet, part_map: &[LetterSet]) -> Option<u32> {
    LETTER_TABLE.iter().enumerate().find_map(|s| {
        if lt.could_map_to(s.1, part_map) {
            Some(s.0 as u32)
        } else{
            None
        }
    })
}

fn letter_to_digit(lt: LetterSet) -> u32 {
    LETTER_TABLE.iter().enumerate().filter_map(|v| {
        if *v.1 == lt {
            Some(v.0 as u32)
        } else {
            None
        }
    }).next().unwrap()
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
                    Some(e.chars().fold(LetterSet::empty(), |acc, c| {
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
    let mut sum = 0;
    for entry in data.iter() {
        let mut map_guess = [A; 7];
        let mut guess_pos = 0;
        while guess_pos != 7 {
            if entry.iter().all(|v| {
                could_match_digit(*v, &map_guess[..=guess_pos])
            }) {
                guess_pos += 1;
            } else {
                loop {
                    match map_guess[guess_pos].next() {
                        Some(next) => {
                            map_guess[guess_pos] = next;
                            break;
                        },
                        None => {
                            map_guess[guess_pos] = A;
                            guess_pos -= 1;
                        }
                    }
                }
            }
        }
        sum += entry.iter().skip(10)
            .fold(0, |acc, v| {
                acc * 10 + letter_to_digit(v.map(&map_guess))
            })
    }
    println!("2> {}", sum);
}

fn main() {
    let data = read_data().collect::<Vec<_>>();
    p1(&data);
    p2(&data);
}
