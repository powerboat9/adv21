#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(mixed_integer_ops)]
#![feature(cell_update)]
#[macro_use]
extern crate lazy_static;

use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{from_fn, Peekable};
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

const FILENAME: &'static str = "i1.txt";

fn chunk_count<V, const N: usize>(mut it: impl Iterator<Item=V>) -> impl Iterator<Item=[V; N]> {
    from_fn(move || {
        let mut ret = MaybeUninit::uninit_array();
        for pos in 0..N {
            if pos == 0 {
                ret[pos] = MaybeUninit::new(it.next()?);
            } else {
                ret[pos] = MaybeUninit::new(it.next().unwrap());
            }
        }
        unsafe {
            Some(MaybeUninit::array_assume_init(ret))
        }
    })
}

fn take_number(it: &mut Peekable<impl Iterator<Item=char>>) -> Option<u32> {
    let mut acc = 0;
    let has_n = false;
    while let Some(c) = it.next_if(|v| v.is_ascii_digit()) {
        acc *= 10;
        acc += (c as u32) - ('0' as u32);
    }
    if has_n {
        Some(acc)
    } else {
        None
    }
}

fn take_white_numbers(it: &mut Peekable<impl Iterator<Item=char>>) -> Vec<u32> {
    from_fn(|| {
        take_ignore_white(it, take_number)
    }).collect()
}

fn map_nonwhite_chunks(mut it: impl Iterator<Item=char>) -> impl Iterator<Item=String> {
    let mut acc = None;
    let mut cont = true;
    from_fn(move || {
        if !cont {
            return None
        }
        loop {
            match it.next() {
                Some(c) if c.is_ascii_whitespace() => {
                    if acc.is_some() {
                        return acc.take();
                    }
                },
                Some(c) => {
                    acc.get_or_insert(String::new()).push(c);
                }
                None => {
                    cont = false;
                    return acc.take()
                }
            }
        }
    })
}

fn take_ignore_white<I: Iterator<Item=char>, V>(it: &mut Peekable<I>, tk_fn: impl FnOnce(&mut Peekable<I>) -> V) -> V {
    take_any_white(it);
    (tk_fn)(it)
}

fn take_any_white(it: &mut Peekable<impl Iterator<Item=char>>) {
    while let Some(_) = it.next_if(|v| v.is_ascii_whitespace()) {
    }
}

fn expect_char(it: &mut Peekable<impl Iterator<Item=char>>, c: char) -> Result<(), ()> {
    it.next_if_eq(&c).map(|v| ()).ok_or(())
}

fn take_nonwhite(it: &mut Peekable<impl Iterator<Item=char>>) -> Option<String> {
    let mut s = String::new();
    while let Some(c) = it.next_if(|v| v.is_ascii_whitespace()) {
        s.push(c)
    }
    if s.len() != 0 {
        Some(s)
    } else {
        None
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

fn read_data() -> Vec<Vec<(PType, Action)>> {
    read_lines().map(|v| v.chars().filter_map(|c| {
        Some(match c {
            '(' => (PType::Paren, Action::Open),
            '[' => (PType::Square, Action::Open),
            '{' => (PType::Curly, Action::Open),
            '<' => (PType::Arrow, Action::Open),
            ')' => (PType::Paren, Action::Close),
            ']' => (PType::Square, Action::Close),
            '}' => (PType::Curly, Action::Close),
            '>' => (PType::Arrow, Action::Close),
            _ => return None
        })
    }).collect()).collect()
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum PType {
    Paren,
    Square,
    Curly,
    Arrow
}

#[derive(Copy, Clone)]
enum Action {
    Open,
    Close
}

fn main() {
    let mut data = read_data();

    let mut res_1 = 0;
    let mut line_scores = Vec::new();
    'line_it: for line in data.iter() {
        let mut queue = Vec::new();
        for ent in line.iter() {
            match ent.1 {
                Action::Open => {
                    queue.push(ent.0)
                },
                Action::Close => {
                    if queue.pop() != Some(ent.0) {
                        res_1 += match ent.0 {
                            PType::Paren => 3,
                            PType::Square => 57,
                            PType::Curly => 1197,
                            PType::Arrow => 25137
                        };
                        continue 'line_it;
                    }
                }
            }
        }
        let mut score = 0usize;
        while let Some(c) = queue.pop() {
            score *= 5;
            score += match c {
                PType::Paren => 1,
                PType::Square => 2,
                PType::Curly => 3,
                PType::Arrow => 4
            };
        }
        line_scores.push(score);
    }
    line_scores.sort();
    println!("1> {}", res_1);
    println!("2> {}", line_scores[line_scores.len() / 2]);
}
