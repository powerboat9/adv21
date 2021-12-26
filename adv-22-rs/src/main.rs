#![feature(linked_list_cursors)]

use std::collections::{HashMap, HashSet, LinkedList};
use std::collections::hash_map::Entry;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::iter::from_fn;
use std::ops::{Deref, RangeInclusive, RangeToInclusive};
use std::rc::{Rc, Weak};
use std::str::Chars;

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

fn parse_line(s: &str) -> (bool, Rect) {
    fn parse_eq(s: &str) -> (i32, i32) {
        let s = &s[2..];
        let (a, b) = s.split_once("..").unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }

    let (state, s) = if s.starts_with("on ") {
        (true, &s[3..])
    } else if s.starts_with("off ") {
        (false, &s[4..])
    } else {
        panic!("parse failure")
    };
    let (x, yz) = s.split_once(',').unwrap();
    let (y, z) = yz.split_once(',').unwrap();
    let x = parse_eq(x);
    let y = parse_eq(y);
    let z = parse_eq(z);
    (
        state,
        Rect {
            x,
            y,
            z
        }
    )
}

fn read_data() -> impl Iterator<Item=(bool, Rect)> {
    read_lines().map(|line| {
        parse_line(line.as_str())
    })
}

#[derive(Copy, Clone)]
struct Rect {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32)
}

impl Rect {
    fn volume(&self) -> u64 {
        let x = (self.x.1 - self.x.0) as u64 + 1;
        let y = (self.y.1 - self.y.0) as u64 + 1;
        let z = (self.z.1 - self.z.0) as u64 + 1;
        x * y * z
    }
}

fn intersect_1d(a: (i32, i32), b: (i32, i32)) -> Option<(i32, i32)> {
    let v = (a.0.max(b.0), a.1.min(b.1));
    if v.0 > v.1 {
        None
    } else {
        Some(v)
    }
}

fn intersect(a: &Rect, b: &Rect) -> Option<Rect> {
    Some(Rect {
        x: intersect_1d(a.x, b.x)?,
        y: intersect_1d(a.y, b.y)?,
        z: intersect_1d(a.z, b.z)?
    })
}

fn without(a: &Rect, rem: &Rect) -> Vec<Rect> {
    let rem = match intersect(a, rem) {
        Some(v) => v,
        None => return vec![*a]
    };
    let mut ls = Vec::new();
    let mut a = *a;
    if a.x.0 < rem.x.0 {
        ls.push(Rect {
            x: (a.x.0, rem.x.0 - 1),
            y: a.y,
            z: a.z
        });
        a.x.0 = rem.x.0;
    }
    if a.x.1 > rem.x.1 {
        ls.push(Rect {
            x: (rem.x.1 + 1, a.x.1),
            y: a.y,
            z: a.z
        });
        a.x.1 = rem.x.1;
    }
    if a.y.0 < rem.y.0 {
        ls.push(Rect {
            x: a.x,
            y: (a.y.0, rem.y.0 - 1),
            z: a.z
        });
        a.y.0 = rem.y.0;
    }
    if a.y.1 > rem.y.1 {
        ls.push(Rect {
            x: a.x,
            y: (rem.y.1 + 1, a.y.1),
            z: a.z
        });
        a.y.1 = rem.y.1;
    }
    if a.z.0 < rem.z.0 {
        ls.push(Rect {
            x: a.x,
            y: a.y,
            z: (a.z.0, rem.z.0 - 1)
        });
        a.z.0 = rem.z.0;
    }
    if a.z.1 > rem.z.1 {
        ls.push(Rect {
            x: a.x,
            y: a.y,
            z: (rem.z.1 + 1, a.z.1)
        });
    }
    ls
}

fn filter_p1(a: &Rect) -> Option<Rect> {
    intersect(a, &Rect {
        x: (-50, 50),
        y: (-50, 50),
        z: (-50, 50)
    })
}

fn remove_intersecting(ls: &mut LinkedList<Rect>, rem: &Rect) {
    let mut cursor = ls.cursor_front_mut();
    loop {
        let new_rects = if let Some(cur) = cursor.current() {
            without(cur, rem)
        } else {
            break
        };
        cursor.remove_current();
        for a in new_rects {
            cursor.insert_before(a);
        }
    }
}

fn set_intersecting(ls: &mut LinkedList<Rect>, r: (bool, Rect)) {
    remove_intersecting(ls, &r.1);
    if r.0 {
        ls.push_front(r.1)
    }
}

fn volume_within_p1(rect: &Rect) -> u64 {
    filter_p1(rect).map(|v| v.volume()).unwrap_or(0)
}

fn main() {
    let mut p1_ls = LinkedList::new();

    for (state, rect) in read_data() {
        set_intersecting(&mut p1_ls, (state, rect));
    }

    let res_p1 = p1_ls.iter().map(|r| volume_within_p1(r)).sum::<u64>();
    let res_p2 = p1_ls.iter().map(|r| r.volume()).sum::<u64>();
    println!("1> {}", res_p1);
    println!("2> {}", res_p2);
}
