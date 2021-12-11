#![feature(mixed_integer_ops)]
#![feature(bool_to_option)]

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

fn read_data() -> Vec<Vec<u8>> {
    read_lines().map(|v| v.chars().filter_map(|c| {
        Some(match c {
            '0'..='9' => (c as u8) - b'0',
            _ => return None
        })
    }).collect()).collect()
}

fn get_adjacent(y: usize, x: usize) -> impl Iterator<Item=(usize, usize)> {
    (-1..=1).into_iter().map(|y_off| {
        (-1..=1).into_iter().map(move |x_off| (y_off, x_off))
    })
        .flatten().filter(|v| (v.0 != 0) || (v.1 != 0))
        .filter_map(move |v| {
            Some((
                y.checked_add_signed(v.0)?,
                x.checked_add_signed(v.1)?
            ))
        })
        .filter(|v| (v.0 < 10) && (v.1 < 10))
}

fn tick(data: &mut Vec<Vec<u8>>) -> u32 {
    fn inc_pos(data: &mut Vec<Vec<u8>>, y: usize, x: usize) {
        match data[y][x] {
            0..=8 => data[y][x] += 1,
            9 => {
                data[y][x] = 10;
                get_adjacent(y, x).for_each(|v| inc_pos(data, v.0, v.1))
            },
            _ => {}
        }
    }

    for y in 0..10 {
        for x in 0..10 {
            inc_pos(data, y, x)
        }
    }

    let mut ret = 0;
    for y in 0..10 {
        for x in 0..10 {
            if data[y][x] == 10 {
                ret += 1;
                data[y][x] = 0;
            }
        }
    }
    ret
}

fn tick_it(mut data: Vec<Vec<u8>>) -> impl Iterator<Item=(usize, u32)> {
    from_fn(move || Some(tick(&mut data))).enumerate().map(|v| (v.0 + 1, v.1))
}

fn main() {
    let data = read_data();

    let mut res_1 = 0;
    let mut res_2 = 0;
    for (pos, n) in tick_it(data) {
        let mut did_op = false;
        if pos <= 100 {
            did_op = true;
            res_1 += n;
        }
        if res_2 == 0 {
            did_op = true;
            if n == 100 {
                res_2 = pos;
            }
        }
        if !did_op {
            break
        }
    }
    println!("1> {}", res_1);
    println!("2> {}", res_2);
}
