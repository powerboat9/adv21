#![feature(linked_list_cursors)]
#![feature(int_abs_diff)]
#![feature(map_first_last)]

mod p1;
mod p2;
mod common;

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::collections::btree_map::Entry;
use std::fmt::Arguments;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::iter::from_fn;
use std::mem::swap;
use std::thread::Builder;
use crate::Color::Copper;
use crate::common::Color;

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

fn read_data() -> ([Option<Color>; 7], [Option<Color>; 8]) {
    let mut rooms = [None; 8];
    let mut iter = read_lines();
    iter.next().unwrap();
    iter.next().unwrap();
    let line1 = iter.next().unwrap();
    let line2 = iter.next().unwrap();

    line1
        .chars()
        .chain(line2.chars())
        .filter_map(|c| Some(match c {
            'A' => Color::Amber,
            'B' => Color::Bronze,
            'C' => Color::Copper,
            'D' => Color::Desert,
            _ => None?
        }))
        .enumerate()
        .for_each(|(idx, c)| {
            rooms[idx] = Some(c);
        });

    ([None; 7], rooms)
}

fn p2_alter_rooms(rooms: [Option<Color>; 8]) -> [Option<Color>; 16] {
    let mut ret = [None; 16];
    for i in 0..4 {
        ret[i] = rooms[i]
    }
    for (i, e) in [
        Color::Desert, Color::Copper, Color::Bronze, Color::Amber,
        Color::Desert, Color::Bronze, Color::Amber, Color::Copper
    ].into_iter().enumerate() {
        ret[i + 4] = Some(e)
    }
    for i in 4..8 {
        ret[i + 8] = rooms[i]
    }
    for i in 0..4 {
        println!("@ {:?}", &ret[(i*4)..][..4])
    }
    ret
}

fn main() {
    let data = read_data();

    println!("1> {}", p1::best_score(data.0, data.1));

    let data = (data.0, p2_alter_rooms(data.1));

    println!("2> {}", p2::best_score(data.0, data.1));
}
