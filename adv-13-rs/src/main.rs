use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::from_fn;
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

fn read_data() -> (HashSet<(u32, u32)>, Vec<Fold>) {
    let mut it = read_lines();
    let mut map = HashSet::new();
    let mut folds = Vec::new();
    while let Some(line) = it.next() {
        let (a, b) = match line.split_once(',') {
            Some(v) => v,
            None => break
        };
        let b = b.strip_suffix('\n').unwrap_or(b);
        map.insert((a.parse().unwrap(), b.parse().unwrap()));
    }
    for line in it {
        let (a, b) = line.split_once('=').unwrap();
        let b = b.strip_suffix('\n').unwrap_or(b);
        if a.ends_with('x') {
            folds.push(Fold::Left(b.parse().unwrap()))
        } else {
            folds.push(Fold::Up(b.parse().unwrap()))
        }
    }
    (map, folds)
}

fn reflect(a: u32, r: u32) -> u32 {
    if a > r {
        2 * r - a
    } else {
        a
    }
}

fn apply_fold(old_map: &HashSet<(u32, u32)>, fold: Fold) -> HashSet<(u32, u32)> {
    match fold {
        Fold::Up(n) => {
            old_map.iter().map(|(x, y)| (*x, reflect(*y, n))).collect()
        }
        Fold::Left(n) => {
            old_map.iter().map(|(x, y)| (reflect(*x, n), *y)).collect()
        }
    }
}

fn get_dims(map: &HashSet<(u32, u32)>) -> (u32, u32) {
    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in map.iter().copied() {
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    (max_x + 1, max_y + 1)
}

struct Cursor {
    pos: Option<(u32, u32)>,
    dims: (u32, u32)
}

impl Cursor {
    fn new(dims: (u32, u32)) -> Self {
        Cursor {
            pos: Some((0, 0)),
            dims
        }
    }

    fn advance_to(&mut self, pos: Option<(u32, u32)>) {
        while self.pos != pos {
            let (mut s_x, mut s_y) = self.pos.unwrap();
            print!(".");
            s_x += 1;
            if s_x >= self.dims.0 {
                println!();
                s_y += 1;
                s_x = 0;
            }
            if s_y >= self.dims.1 {
                self.pos = None
            } else {
                self.pos = Some((s_x, s_y))
            }
        }
    }

    fn write_dot(&mut self) {
        match &mut self.pos {
            Some((x, y)) => {
                print!("#");
                *x += 1;
                if *x >= self.dims.0 {
                    println!();
                    *y += 1;
                    *x = 0;
                    if *y < self.dims.1 {
                        return
                    }
                } else {
                    return
                }
            },
            None => panic!()
        }
        self.pos = None
    }
}

fn display_map(map: &HashSet<(u32, u32)>) {
    let (width, height) = get_dims(map);
    let mut ls = Vec::from_iter(map.iter().copied());
    ls.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0))
    });
    let mut cur = Cursor::new((width, height));
    while let Some(d_pos) = ls.pop() {
        cur.advance_to(Some(d_pos));
        cur.write_dot();
    }
    cur.advance_to(None)
}

fn main() {
    let (mut dots, folds) = read_data();

    dots = apply_fold(&dots, *folds.first().unwrap());
    println!("1> {}", dots.len());
    for fold in folds.iter().skip(1).copied() {
        dots = apply_fold(&dots, fold)
    }
    println!("2>");
    display_map(&dots);
}
