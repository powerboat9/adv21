#![feature(bool_to_option)]
#![feature(destructuring_assignment)]

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::iter::{from_fn, Peekable};
use std::mem::swap;
use std::ops::{Index, IndexMut};
use std::panic::catch_unwind;
use bitvec::field::BitField;
use bitvec::mem::BitMemory;
use bitvec::order::{BitOrder, Msb0};
use bitvec::slice::BitSlice;
use bitvec::store::BitStore;
use bitvec::vec::BitVec;

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

fn read_data() -> BitVec<Msb0> {
    read_lines()
        .next()
        .unwrap()
        .bytes()
        .map(|c| {
            match c {
                b'0'..=b'9' => {
                    Some(c - b'0')
                }
                b'A'..=b'F' => {
                    Some((c - b'A') + 10)
                }
                _ => None
            }
        })
        .filter_map(|v| v)
        .flat_map(|v| {
            (0..4).rev().map(move |n| {
                ((v >> n) & 1) != 0
            })
        })
        .collect()
}

struct PacketOp {
    version: u8,
    type_id: u8,
    ls: Vec<Packet>
}

struct PacketLit {
    version: u8,
    val: u64
}

enum Packet {
    Op(PacketOp),
    Lit(PacketLit)
}

impl Packet {
    fn get_version_sum(&self) -> u32 {
        match self {
            Packet::Op(o) => o.ls.iter().map(Packet::get_version_sum).sum::<u32>() + (o.version as u32),
            Packet::Lit(l) => l.version as u32
        }
    }

    fn exec(&self) -> u64 {
        match self {
            Packet::Lit(l) => l.val,
            Packet::Op(p) => {
                 match p.type_id {
                     0 => p.ls.iter().map(Packet::exec).sum(),
                     1 => p.ls.iter().map(Packet::exec).product(),
                     2 => p.ls.iter().map(Packet::exec).min().unwrap(),
                     3 => p.ls.iter().map(Packet::exec).max().unwrap(),
                     5 => (p.ls[0].exec() > p.ls[1].exec()) as u64,
                     6 => (p.ls[0].exec() < p.ls[1].exec()) as u64,
                     7 => (p.ls[0].exec() == p.ls[1].exec()) as u64,
                     _ => unreachable!()
                 }
            }
        }
    }
}

fn parse_all_packets<T: BitStore>(mut v: &BitSlice<Msb0, T>) -> Vec<Packet> {
    from_fn(move || {
        if v.len() != 0 {
            let (p, new_v) = parse_single_packet(v);
            v = new_v;
            Some(p)
        } else {
            None
        }
    }).collect()
}

fn parse_n_packets<T: BitStore>(mut v: &BitSlice<Msb0, T>, n: usize) -> (Vec<Packet>, &BitSlice<Msb0, T>) {
    let ls = (0..n).map(|_| {
        let (p, new_v) = parse_single_packet(v);
        v = new_v;
        p
    }).collect();
    (
        ls,
        v
    )
}

fn parse_single_packet<T: BitStore>(mut v: &BitSlice<Msb0, T>) -> (Packet, &BitSlice<Msb0, T>) {
    let version = v[0..3].load_be::<u8>();
    let type_id = v[3..6].load_be::<u8>();
    v = &v[6..];
    println!("[ ({}, {})", version, type_id);
    if type_id == 4 {
        let mut acc = 0;
        loop {
            let should_stop = !v[0];
            let n = v[1..5].load_be::<u8>();
            acc = (acc << 4) | (n as u64);
            v = &v[5..];
            if should_stop {
                break
            }
        }
        (
            Packet::Lit(PacketLit {
                version,
                val: acc
            }),
            v
        )
    } else {
        if v[0] {
            let n = v[1..12].load_be();
            let (ls, rest) =
                parse_n_packets(&v[12..], n);
            println!("]");
            (
                Packet::Op(PacketOp {
                    version,
                    type_id,
                    ls
                }),
                rest
            )
        } else {
            let size = v[1..16].load_be();
            v = &v[16..];
            let ls = parse_all_packets(&v[..size]);
            println!("]");
            (
                Packet::Op(PacketOp {
                    version,
                    type_id,
                    ls
                }),
                &v[size..]
            )
        }
    }
}

fn parse_packet<T:BitStore>(v: &BitSlice<Msb0, T>) -> Packet {
    parse_single_packet(v).0
}

fn main() {
    let data = read_data();
    let packets = parse_packet(data.as_bitslice());

    println!("1> {}", packets.get_version_sum());
    println!("2> {}", packets.exec());
}
