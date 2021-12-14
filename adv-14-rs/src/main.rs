use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::iter::from_fn;
use std::mem::swap;
use std::ops::BitAnd;

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

#[derive(Copy, Clone)]
enum Fold {
    Up(u32),
    Left(u32)
}

fn read_data() -> (Vec<char>, HashMap<(char, char), char>) {
    let mut it = read_lines();
    let mut init_string = it.next().unwrap().chars().collect::<Vec<_>>();
    init_string.pop().unwrap(); // last newline
    it.next().unwrap();
    let mut map = HashMap::new();
    for line in it {
        let (a, b) = line.split_once(" -> ").unwrap();
        let a1 = a.chars().next().unwrap();
        let a2 = a.chars().skip(1).next().unwrap();
        map.insert((a1, a2), b.chars().next().unwrap());
    }
    (init_string, map)
}

fn inc_by<T: Eq + Hash>(map: &mut HashMap<T, usize>, k: T, v: usize) {
    match map.entry(k) {
        Entry::Occupied(mut o) => {
            *o.get_mut() += v;
        }
        Entry::Vacant(e) => {
            e.insert(v);
        }
    }
}

struct Chain {
    links: HashMap<(Option<char>, Option<char>), usize>
}

impl Chain {
    fn init(chain: &[char]) -> Self {
        if chain.len() == 0 {
            Chain {
                links: HashMap::new()
            }
        } else {
            let mut links = HashMap::new();
            links.insert((None, Some(chain[0])), 1);
            for i in 0..(chain.len() - 1) {
                inc_by(&mut links, (Some(chain[i]), Some(chain[i+1])), 1);
            }
            links.insert((Some(chain[chain.len() - 1]), None), 1);
            Chain {
                links
            }
        }
    }

    fn apply_map(&mut self, map: &HashMap<(char, char), char>) {
        let mut old_map = HashMap::new();
        swap(&mut self.links, &mut old_map);
        for (old_end, n) in old_map.into_iter() {
            if let (Some(a), Some(b)) = old_end {
                if let Some(ins) = map.get(&(a, b)) {
                    inc_by(&mut self.links, (Some(a), Some(*ins)), n);
                    inc_by(&mut self.links, (Some(*ins), Some(b)), n);
                } else {
                    inc_by(&mut self.links, (Some(a), Some(b)), n);
                }
            } else {
                self.links.insert((old_end.0, old_end.1), n);
            }
        }
    }

    fn get_chain_score(&self) -> usize {
        let mut char_refs = HashMap::new();
        for (ent, n) in self.links.iter() {
            if let Some(c) = ent.0 {
                inc_by(&mut char_refs, c, *n);
            }
            if let Some(c) = ent.1 {
                inc_by(&mut char_refs, c, *n);
            }
        }
        let mut min = None;
        let mut max = None;
        for (c, n) in char_refs.into_iter() {
            match min {
                Some((_, n2)) if n2 < n => {},
                _ => min = Some((c, n))
            }
            match max {
                Some((_, n2)) if n2 > n => {},
                _ => max = Some((c, n))
            }
        }
        (max.unwrap().1 - min.unwrap().1) / 2
    }
}

fn get_char_max(chain: &[char]) -> u32 {
    let mut track = HashMap::new();
    for c in chain {
        *track.entry(*c).or_insert(0) += 1;
    }
    track.into_iter().max_by(|(_, a), (_, b)| {
        a.cmp(b)
    }).unwrap().1
}

fn get_char_min(chain: &[char]) -> u32 {
    let mut track = HashMap::new();
    for c in chain {
        *track.entry(*c).or_insert(0) += 1;
    }
    track.into_iter().min_by(|(_, a), (_, b)| {
        a.cmp(b)
    }).unwrap().1
}

fn main() {
    let (chain, map) = read_data();
    let mut chain = Chain::init(chain.as_slice());

    for _ in 0..10 {
        chain.apply_map(&map)
    }
    println!("1> {}", chain.get_chain_score());
    for _ in 0..30 {
        chain.apply_map(&map)
    }
    println!("2> {}", chain.get_chain_score());
}
