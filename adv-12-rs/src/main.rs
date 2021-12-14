#![feature(mixed_integer_ops)]
#![feature(bool_to_option)]

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{from_fn};

const FILENAME: &'static str = "i1.txt";

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

fn read_data() -> (usize, usize, Vec<(usize, usize)>, Vec<bool>) {
    let mut ret = Vec::new();
    let mut can_multi = Vec::new();
    let mut id_map = HashMap::new();
    let mut next_id = 0;
    for line in read_lines() {
        let (a, b) = line.split_once('-').unwrap();
        let b = b.strip_suffix('\n').unwrap_or(b);
        let a = String::from(a);
        let b = String::from(b);
        let a_id = *id_map.entry(a).or_insert_with_key(|a| {
            let n = next_id;
            next_id += 1;
            can_multi.push(a.chars().next().unwrap().is_uppercase());
            n
        });
        let b_id = *id_map.entry(b).or_insert_with_key(|b| {
            let n = next_id;
            next_id += 1;
            can_multi.push(b.chars().next().unwrap().is_uppercase());
            n
        });
        ret.push((a_id, b_id));
        ret.push((b_id, a_id));
    }
    (
        *id_map.get("start").unwrap(),
        *id_map.get("end").unwrap(),
        ret,
        can_multi
    )
}

#[derive(Copy, Clone)]
enum VisitState {
    CanTwice,
    Needs(usize),
    NoTwice
}

impl VisitState {
    fn is_needy(&self) -> bool {
        if let VisitState::Needs(_) = self {
            true
        } else {
            false
        }
    }

    fn next_states<'a>(
        &self, from: usize,
        is_start: bool,
        paths: &'a [(usize, usize)],
        can_multi: bool
    ) -> impl Iterator<Item=(Self, Cow<'a, [(usize, usize)]>)> {
        if can_multi {
            vec![
                (*self, Cow::from(paths))
            ]
        } else if is_start {
            vec![
                (*self, Cow::from(without(paths, from)))
            ]
        } else {
            match self {
                VisitState::CanTwice => {
                    vec![
                        (VisitState::CanTwice, Cow::from(without(paths, from))),
                        (VisitState::Needs(from), Cow::from(paths))
                    ]
                },
                VisitState::Needs(n) => {
                    vec![
                        (
                            if *n == from {
                                VisitState::NoTwice
                            } else {
                                VisitState::Needs(*n)
                            },
                            Cow::from(without(paths, from))
                        )
                    ]
                }
                VisitState::NoTwice => {
                    vec![
                        (VisitState::NoTwice, Cow::from(without(paths, from)))
                    ]
                }
            }
        }.into_iter()
    }
}

fn reachable_from<'a>(
    paths: &'a [(usize, usize)],
    n: usize
) -> impl 'a + Iterator<Item=usize> {
    paths.iter()
        .filter_map(move |v| {
            if v.0 == n {
                Some(v.1)
            } else {
                None
            }
        })
}

fn without(paths: &[(usize, usize)], n: usize) -> Vec<(usize, usize)> {
    paths.iter().copied().filter(|v| {
        (v.0 != n) && (v.1 != n)
    }).collect()
}

fn find_path_cnt(
    start: usize,
    end: usize,
    paths: &[(usize, usize)],
    can_multi: &[bool],
    state: VisitState,
    is_start: bool
) -> usize {
    if start == end {
        return if state.is_needy() {
            0
        } else {
            1
        }
    }

    let mut sum = 0;
    for to in reachable_from(paths, start) {
        for (next_state, next_paths) in
        state.next_states(start, is_start, paths, can_multi[start]) {
            sum += find_path_cnt(
                to, end,
                next_paths.as_ref(), can_multi,
                next_state,
                false
            )
        }
    }
    sum
}

fn main() {
    let (start, end, paths, can_multi) = read_data();

    println!("1> {}", find_path_cnt(start, end, paths.as_slice(), can_multi.as_slice(), VisitState::NoTwice, true));
    println!("2> {}", find_path_cnt(start, end, paths.as_slice(), can_multi.as_slice(), VisitState::CanTwice, true));
}
