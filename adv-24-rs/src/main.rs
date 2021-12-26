#![feature(linked_list_cursors)]
#![feature(int_abs_diff)]
#![feature(map_first_last)]

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;
use std::fmt::Arguments;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::iter::from_fn;
use std::mem::swap;
use std::ops::Rem;
use std::process::exit;
use std::thread::Builder;

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

fn parse_reg(s: &str) -> Option<u8> {
    Some(match s {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => return None
    })
}

fn parse_rval(s: &str) -> RVal {
    match parse_reg(s) {
        Some(r) => RVal::Reg(r),
        None => RVal::Val(s.parse().unwrap())
    }
}

fn parse_ab(s: &str) -> (u8, RVal) {
    let (a, b) = s.split_once(' ').unwrap();
    let a = parse_reg(a).unwrap();
    let b = parse_rval(b);
    (a, b)
}

fn parse_ins(s: &str) -> Ins {
    let body = &s[4..];
    match &s[..4] {
        "inp " => Ins::Inp(parse_reg(body).unwrap()),
        _ => {
            let (a, b) = parse_ab(body);
            Ins::Reg(RegIns {
                t: match &s[..4] {
                    "add " => RegInsType::Add,
                    "mul " => RegInsType::Mul,
                    "div " => RegInsType::Div,
                    "mod " => RegInsType::Mod,
                    "eql " => RegInsType::Eql,
                    _ => unreachable!()
                },
                a,
                b
            })
        }
    }
}

fn read_data() -> impl Iterator<Item=Ins> {
    read_lines()
        .map(|l| parse_ins(l.as_str()))
}

#[derive(Copy, Clone)]
enum RVal {
    Reg(u8),
    Val(i64),
}

#[derive(Copy, Clone)]
enum RegInsType {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl RegInsType {
    fn exec(&self, regs: &mut [i64; 4], a: u8, b: RVal) {
        let b = match b {
            RVal::Reg(r) => regs[r as usize],
            RVal::Val(n) => n
        };
        regs[a as usize] = match self {
            RegInsType::Add => regs[a as usize] + b,
            RegInsType::Mul => regs[a as usize] * b,
            RegInsType::Div => regs[a as usize] / b,
            RegInsType::Mod => regs[a as usize].rem_euclid(b),
            RegInsType::Eql => (regs[a as usize] == b) as i64
        }
    }
}

#[derive(Copy, Clone)]
struct RegIns {
    t: RegInsType,
    a: u8,
    b: RVal,
}

impl RegIns {
    fn exec(&self, regs: &mut [i64; 4]) {
        self.t.exec(regs, self.a, self.b)
    }
}

#[derive(Copy, Clone)]
enum Ins {
    Inp(u8),
    Reg(RegIns),
}

impl Ins {
    fn unwrap_reg(&self) -> RegIns {
        if let Ins::Reg(r) = self {
            *r
        } else {
            panic!("unwrap failure")
        }
    }

    fn unwrap_inp(&self) -> u8 {
        if let Ins::Inp(r) = self {
            *r
        } else {
            panic!("unwrap failure")
        }
    }

    fn is_reg(&self) -> bool {
        match self {
            Ins::Inp(_) => false,
            Ins::Reg(_) => true
        }
    }
}

#[derive(Clone)]
struct Game {
    regs: [i32; 4],
}

fn collect_big(it: impl Iterator<Item=([i64; 4], u64)>) -> HashMap<[i64; 4], u64> {
    let mut h = HashMap::new();
    for (regs, n) in it {
        match h.entry(regs) {
            Entry::Occupied(mut e) => {
                let e = e.get_mut();
                if *e < n {
                    *e = n;
                }
            }
            Entry::Vacant(e) => {
                e.insert(n);
            }
        }
    }
    h
}

fn collect_small(it: impl Iterator<Item=([i64; 4], u64)>) -> HashMap<[i64; 4], u64> {
    let mut h = HashMap::new();
    for (regs, n) in it {
        match h.entry(regs) {
            Entry::Occupied(mut e) => {
                let e = e.get_mut();
                if *e > n {
                    *e = n;
                }
            }
            Entry::Vacant(e) => {
                e.insert(n);
            }
        }
    }
    h
}

fn split_reg_ins(ls: &[Ins]) -> (&[Ins], &[Ins]) {
    let mut i = 0;
    loop {
        if i >= ls.len() {
            return (ls, &[])
        } else if !ls[i].is_reg() {
            return ls.split_at(i)
        }
        i += 1;
    }
}

fn p1(mut ls: &[Ins]) -> u64 {
    let mut map = HashMap::new();
    map.insert([0; 4], 0);
    while ls.len() != 0 {
        //println!("FOO: {}", map.len());
        let (exec, new_ls) = split_reg_ins(ls);
        if exec.len() != 0 {
            ls = new_ls;
            map = collect_big(
                map
                    .into_iter()
                    .map(|mut v| {
                        for ins in exec {
                            ins.unwrap_reg().exec(&mut v.0);
                        }
                        v
                    })
            )
        } else {
            let r = ls[0].unwrap_inp();
            ls = &ls[1..];
            map = collect_big(
                map
                    .into_iter()
                    .flat_map(|v| {
                        (1..=9)
                            .map(move |i| {
                                let mut new_regs = v.0;
                                new_regs[r as usize] = i;
                                (new_regs, v.1 * 10 + (i as u64))
                            })
                    }))
        }
    }
    map
        .into_iter()
        .filter_map(|v| {
            if v.0[3] == 0 {
                Some(v.1)
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

fn p2(mut ls: &[Ins]) -> u64 {
    let mut map = HashMap::new();
    map.insert([0; 4], 0);
    while ls.len() != 0 {
        //println!("FOO: {}", map.len());
        let (exec, new_ls) = split_reg_ins(ls);
        if exec.len() != 0 {
            ls = new_ls;
            map = collect_small(
                map
                    .into_iter()
                    .map(|mut v| {
                        for ins in exec {
                            ins.unwrap_reg().exec(&mut v.0);
                        }
                        v
                    })
            )
        } else {
            let r = ls[0].unwrap_inp();
            ls = &ls[1..];
            map = collect_small(
                map
                    .into_iter()
                    .flat_map(|v| {
                        (1..=9)
                            .map(move |i| {
                                let mut new_regs = v.0;
                                new_regs[r as usize] = i;
                                (new_regs, v.1 * 10 + (i as u64))
                            })
                    }))
        }
    }
    map
        .into_iter()
        .filter_map(|v| {
            if v.0[3] == 0 {
                Some(v.1)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn main() {
    let data = read_data().collect::<Vec<_>>();

    println!("1> {}", p1(data.as_slice()));
    println!("2> {}", p2(data.as_slice()));
}
