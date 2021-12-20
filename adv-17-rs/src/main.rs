#![feature(bool_to_option)]
#![feature(destructuring_assignment)]

#[macro_use]
extern crate scan_fmt;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::from_fn;
use std::ops::Rem;

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

fn read_data() -> (i32, i32, i32, i32) {
    scan_fmt!(read_lines()
        .next()
        .unwrap()
        .as_str(),
        "target area: x={d}..{d}, y={d}..{d}",
        i32, i32, i32, i32
    ).unwrap()
}

fn find_all_plus_divides(mut v: i32) -> impl Iterator<Item=i32> {
    if v < 0 {
        v = -v
    }
    let v = v;
    (1..=v)
        .filter(move |d| {
            v.rem(d) == 0
        })
}

fn try_divide(n: i32, d: i32) -> Option<i32> {
    if n.rem(d) == 0 {
        Some(n / d)
    } else {
        None
    }
}

fn int_sqrt(x: i32) -> Option<i32> {
    match x {
        0 => Some(0),
        1 => Some(1),
        _ => {
            let mut low = 1;
            let mut high = x;
            loop {
                if low == high {
                    return None
                }
                let choose = (low + high) / 2;
                match choose.checked_mul(choose) {
                    None => {
                        high = choose;
                    }
                    Some(c) if c > x => {
                        high = choose;
                    }
                    Some(c) if c == x => {
                        return Some(choose);
                    }
                    _ => {
                        low = choose + 1
                    }
                }
            }
        }
    }
}

fn find_all_hitting(tx: i32, ty: i32) -> Box<dyn Iterator<Item=(i32, i32, i32)>> {
    let dx_mul = tx.signum();
    let tx = tx * dx_mul;
    Box::new(find_all_plus_divides(2 * ty)
        .flat_map(move |n| {
            let stopping_dx = int_sqrt(1 + 8 * tx)
                .and_then(|v| try_divide(v - 1, 2))
                .filter(|m| *m <= n);
            let passing_dx = try_divide(2 * tx + n * (n - 1), 2 * n)
                .filter(|&dx| dx > n);
            passing_dx.into_iter()
                .chain(stopping_dx.into_iter())
                .map(move|dx| (n, dx))
        })
        .filter_map(move |(n, dx)| {
            try_divide(2 * ty + n * (n - 1), 2 * n).map(move |dy| (n, dx, dy))
        })
    )
}

fn find_in_bounds(target: (i32, i32, i32, i32)) -> impl Iterator<Item=(i32, i32)> {
    (target.0..=target.1)
        .flat_map(move |tx| {
            (target.2..=target.3)
                .map(move |ty| {
                    (tx, ty)
                })
        })
}

fn main() {
    let data = read_data();

    let mut p1_res: Option<i32> = None;
    let p2_res =
        find_in_bounds(data)
            .flat_map(|(tx, ty)| find_all_hitting(tx, ty))
            .map(|(_, dx, dy)| (dx, dy))
            .inspect(|(_, dy)| {
                match &mut p1_res {
                    Some(p1_res) => *p1_res = (*p1_res).max(*dy),
                    None => p1_res = Some(*dy)
                }
            })
            .collect::<HashSet<_>>()
            .len();
    let p1_res = p1_res.unwrap();
    let p1_res = p1_res * (p1_res + 1) / 2;

    println!("1> {}", p1_res);
    println!("2> {}", p2_res);
}
