#![feature(str_internals)]
extern crate owning_ref;

use core::str::next_code_point;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::from_fn;
use std::ops::Deref;
use std::str::Chars;
use owning_ref::{OwningHandle, OwningRef};

const FILENAME: &'static str = "i1.txt";

struct Map {
    ls: HashSet<(i32, i32)>,
    is_invert: bool
}

impl Map {
    fn from_light_list(it: impl Iterator<Item=(i32, i32)>) -> Self {
        Map {
            ls: it.collect(),
            is_invert: false
        }
    }

    fn read_pixel(&self, y: i32, x: i32) -> bool {
        self.ls.contains(&(y, x)) ^ self.is_invert
    }

    fn get_blank_id(&self) -> usize {
        if self.is_invert {
            511
        } else {
            0
        }
    }

    fn enhance(&self, f: &impl Fn(usize) -> bool) -> Self {
        let mut invert: bool = f(self.get_blank_id());
        let mut new_img = HashSet::new();
        for &point in self.ls.iter() {
            for y in -1..=1 {
                for x in -1..=1 {
                    let c = (point.0 + y, point.1 + x);
                    if f(self.number_in_area(c)) ^ invert {
                        new_img.insert(c);
                    }
                }
            }
        }
        Map {
            ls: new_img,
            is_invert: invert
        }
    }

    fn number_in_area(&self, pos: (i32, i32)) -> usize {
        let mut acc = 0;
        for y in -1..=1 {
            for x in -1..=1 {
                acc <<= 1;
                acc |= self.read_pixel(pos.0 + y, pos.1 + x) as usize
            }
        }
        acc
    }

    fn get_count(&self) -> usize {
        if self.is_invert {
            panic!("inverted image has infinite light pixels")
        } else {
            self.ls.len()
        }
    }

    fn enhance_count(&mut self, f: &impl Fn(usize) -> bool, n: usize) {
        for _ in 0..n {
            *self = self.enhance(f)
        }
    }
}

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

fn into_bool_array(s: impl Deref<Target=str>) -> Vec<bool> {
    s.chars().map(|c| c == '#').collect()
}

fn read_data() -> (Map, impl Fn(usize) -> bool) {
    let mut it = read_lines();
    let table = into_bool_array(it.next().unwrap());
    it.next().unwrap();
    let img = Map::from_light_list(it
        .enumerate()
        .flat_map(|(row, line)| {
           line
               .chars()
               .enumerate()
               .filter_map(|(col, c)| {
                   if c == '#' {
                       Some((row as i32, col as i32))
                   } else {
                       None
                   }
               })
               .collect::<Vec<_>>()
               .into_iter()
        }));
    (img, move |idx| table[idx])
}

fn number_in_area(img: &HashSet<(i32, i32)>, pos: (i32, i32)) -> usize {
    let mut acc = 0;
    for y in -1..=1 {
        for x in -1..=1 {
            acc <<= 1;
            acc |= img.contains(&(pos.0 + y, pos.1 + x)) as usize
        }
    }
    acc
}

fn main() {
    let (mut img, lookup) = read_data();

    img.enhance_count(&lookup, 2);

    println!("1> {}", img.get_count());

    img.enhance_count(&lookup, 48);

    println!("2> {}", img.get_count());
}
