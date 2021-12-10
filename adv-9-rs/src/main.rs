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

//const FILENAME: &'static str = "bb-9-4096-4.in.txt";
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

fn read_data() -> Vec<Vec<u8>> {
    read_lines().map(|v| v.chars().filter(|c| c.is_ascii_digit()).map(|c| (c as u8) - ('0' as u8)).collect()).collect()
}

fn get_dig_bound(data: &Vec<Vec<u8>>, y: isize, x: isize) -> u8 {
    if (y < 0) || (x < 0) {
        return 255;
    }
    let y = y as usize;
    let x = x as usize;
    data.get(y).and_then(|v| v.get(x)).copied().unwrap_or(255)
}

fn get_adjacent(y: usize, x: usize, y_max: usize, x_max: usize) -> impl Iterator<Item=(usize, usize)> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)].into_iter().filter_map(move |v| {
        let y_pos = y.checked_add_signed(v.0)?;
        let x_pos = x.checked_add_signed(v.1)?;
        if (y_pos >= y_max) || (x_pos >= x_max) {
            None
        } else {
            Some((y_pos, x_pos))
        }
    })
}

fn find_drains_to(data: &Vec<Vec<u8>>, y: usize, x: usize, y_max: usize, x_max: usize) -> HashSet<(usize, usize)> {
    let mut ret = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_front((y, x));
    while let Some(coord) = queue.pop_front() {
        if ret.insert(coord) {
            queue.extend(get_adjacent(coord.0, coord.1, y_max, x_max).filter(|c| {
                let n = data[c.0][c.1];
                (n >= data[coord.0][coord.1]) && (n != 9)
            }));
        }
    }
    ret
}

fn main() {
    let data = read_data();
    let mut low_points = Vec::new();
    let height = data.len();
    let width = data[0].len();

    // part 1
    let mut res_1 = 0;
    for y in 0..height {
        for x in 0..width {
            if get_adjacent(y, x, height, width)
                .all(|v| data[v.0][v.1] > data[y][x])
            {
                res_1 += 1 + (data[y][x] as u32);
                low_points.push((y, x));
            }
        }
    }
    println!("1> {}", res_1);

    // part 2
    let mut max_1 = 0;
    let mut max_2 = 0;
    let mut max_3 = 0;
    for low in low_points {
        let n = find_drains_to(&data, low.0, low.1, height, width).len();
        if n > max_1 {
            max_3 = max_2;
            max_2 = max_1;
            max_1 = n;
        } else if n > max_2 {
            max_3 = max_2;
            max_2 = n;
        } else if n > max_3 {
            max_3 = n;
        }
    }
    println!("2> {}", max_1 * max_2 * max_3);
}
