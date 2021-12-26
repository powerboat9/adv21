#![feature(bool_to_option)]
#![feature(destructuring_assignment)]
#![feature(const_mut_refs)]
#![feature(generic_const_exprs)]

#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::from_fn;
use std::mem::swap;
use std::ops::{Add, Mul, Sub};

const FILENAME: &'static str = "i1.txt";

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Matrix<const R: usize, const C: usize> where [(); R * C]: Sized {
    backing: [i32; R * C]
}

impl<const R: usize, const C: usize> Add for Matrix<R, C> where [(); R * C]: Sized {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = [0; R * C];
        for i in 0..(R * C) {
            ret[i] = self.backing[i] + rhs.backing[i]
        }
        Matrix::<R, C> {
            backing: ret
        }
    }
}

impl<const R: usize, const C: usize> Sub for Matrix<R, C> where [(); R * C]: Sized {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = [0; R * C];
        for i in 0..(R * C) {
            ret[i] = self.backing[i] - rhs.backing[i]
        }
        Matrix::<R, C> {
            backing: ret
        }
    }
}

impl<const RC: usize> Matrix<RC, RC> where [(); RC * RC]: Sized {
    fn transpose(&self) -> Self {
        let mut ret = [0; RC * RC];
        for row in 0..RC {
            for col in 0..RC {
                ret[col * RC + row] = self.backing[row * RC + col]
            }
        }
        Matrix {
            backing: ret
        }
    }
}

fn matrix_multiply<
    const A: usize,
    const B: usize,
    const C: usize>
(
    a: &Matrix<A, B>,
    b: &Matrix<B, C>
) -> Matrix<A, C> where
    [(); A * B]: Sized,
    [(); B * C]: Sized,
    [(); A * C]: Sized
{
    let mut ret = [0; A * C];
    for row in 0..A {
        for col in 0..C {
            ret[row * C + col] = 0;
            for i in 0..B {
                ret[row * C + col] += a.backing[row * B + i] * b.backing[i * C + col];
            }
        }
    }
    Matrix::<A, C> {
        backing: ret
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Vec3i(Matrix<3, 1>);

impl Add for Vec3i {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3i(self.0 + rhs.0)
    }
}

impl Sub for Vec3i {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3i(self.0 - rhs.0)
    }
}

impl Vec3i {
    fn apply_rot(&self, rot: &Rot) -> Self {
        Vec3i(matrix_multiply(&rot.0,  &self.0))
    }

    fn apply_rt(&self, rt: &RotAndTrans) -> Self {
        self.apply_rot(&rt.rot) + rt.trans
    }

    fn new(x: i32, y: i32, z: i32) -> Self {
        Vec3i(Matrix {
            backing: [x, y, z]
        })
    }

    fn man_dist(&self, oth: &Self) -> i32 {
        let mut acc = 0;
        for i in 0..3 {
            acc += (self.0.backing[i] - oth.0.backing[i]).abs()
        }
        acc
    }
}

#[derive(Copy, Clone, Debug)]
struct Rot(Matrix<3, 3>);

impl Mul for Rot {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Rot(matrix_multiply(&self.0, &rhs.0))
    }
}

impl Rot {
    const fn new(m: [i32; 9]) -> Self {
        Rot(Matrix {
            backing: m
        })
    }

    // assumes valid rotation - orthogonal - transpose is inverse
    fn inverse(&self) -> Self {
        Rot(self.0.transpose())
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
            Some(s)
        }
    })
}

fn line_as_coords(s: &str) -> Option<(i32, i32, i32)> {
    let s = s.trim_end();
    let (a, bc) = s.split_once(',')?;
    let (b, c) = bc.split_once(',')?;
    Some((a.parse().ok()?, b.parse().ok()?, c.parse().ok()?))
}

fn read_data() -> Vec<FreeChunk> {
    let mut ret = Vec::new();
    let mut cur = None;
    let mut id = 0;
    for line in read_lines() {
        let line = line_as_coords(line.as_str());
        match line {
            Some(line) => {
                cur
                    .get_or_insert_with(HashSet::new)
                    .insert(
                        Vec3i::new(line.0, line.1, line.2)
                    );
            }
            None => {
                if let Some(cur) = cur.take() {
                    ret.push(FreeChunk::new(cur, id));
                    id += 1;
                }
            }
        }
    }
    if let Some(cur) = cur {
        ret.push(FreeChunk::new(cur, id));
    }
    ret
}

#[derive(Clone)]
struct FreeChunk(HashSet<Vec3i>, i32);
#[derive(Clone)]
struct PlacedChunk(HashSet<Vec3i>, RotAndTrans, i32);

impl FreeChunk {
    fn new(hs: HashSet<Vec3i>, id: i32) -> Self {
        FreeChunk(hs, id)
    }

    fn blank() -> Self {
        FreeChunk(HashSet::new(), -1)
    }

    fn place_origin(self) -> PlacedChunk {
        self.with_rt(RotAndTrans {
            rot: EMPTY_ROTATION,
            trans: Vec3i::new(0, 0, 0)
        })
    }

    fn with_rt(self, rt: RotAndTrans) -> PlacedChunk {
        PlacedChunk(self.0, rt, self.1)
    }

    fn with_rt_from(self, pc: &PlacedChunk, rt: &RotAndTrans) -> PlacedChunk {
        self.with_rt(combine_rt(rt, &pc.1))
    }

    fn attempt_place_by(self, base: &PlacedChunk) -> Result<PlacedChunk, FreeChunk> {
        match try_find_rt(&base.0, &self.0) {
            Some(rt) => {
                Ok(self.with_rt_from(base, &rt))
            },
            None => Err(self)
        }
    }
}

fn combine_placed_chunks<'a>(it: impl Iterator<Item=&'a PlacedChunk>) -> HashSet<Vec3i> {
    it
        .flat_map(|p| {
            println!("OFFSET: {:?}", Vec3i::new(0, 0, 0).apply_rt(&p.1));
            p.0
                .iter()
                .map(move |v| v.apply_rt(&p.1))
        })
        .collect()
}

const EMPTY_ROTATION: Rot = Rot::new([1, 0, 0, 0, 1, 0, 0, 0, 1]);

fn generate_all_rotations() -> [Rot; 24] {
    let mut ret = [Rot(Matrix {backing: [0;9]}); 24];

    [
        // x
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        // -y
        [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
        // -x
        [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
        // y
        [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
        // -z
        [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
        // z
        [[0, 0, 1], [0, 1, 0], [-1, 0, 0]]
    ]
        .into_iter()
        .flat_map(|axis_rot| {
            // rotate by 1, -1
            let oth = [
                axis_rot[0],
                [-axis_rot[1][0], -axis_rot[1][1], -axis_rot[1][2]],
                [-axis_rot[2][0], -axis_rot[2][1], -axis_rot[2][2]]
            ];
            [axis_rot, oth].into_iter()
        })
        .flat_map(|axis_rot| {
            // rotate by 1, i
            let oth = [
                axis_rot[0],
                [-axis_rot[2][0], -axis_rot[2][1], -axis_rot[2][2]],
                [axis_rot[1][0], axis_rot[1][1], axis_rot[1][2]]
            ];
            [axis_rot, oth].into_iter()
        })
        .map(|rot| Rot(Matrix {
            backing: [
                rot[0][0], rot[0][1], rot[0][2],
                rot[1][0], rot[1][1], rot[1][2],
                rot[2][0], rot[2][1], rot[2][2]
            ]
        }))
        .enumerate()
        .for_each(|(idx, r)| ret[idx] = r);

    ret
}

lazy_static! {
    static ref ALL_ROTATIONS: [Rot; 24] = generate_all_rotations();
}

#[derive(Copy, Clone, Debug)]
struct Rotation([[i32; 3]; 3]);

#[derive(Copy, Clone, Debug)]
struct RotAndTrans {
    rot: Rot,
    trans: Vec3i,
}

impl RotAndTrans {
    fn invert(&self) -> Self {
        let rot = self.rot.inverse();
        let trans = self.trans.apply_rot(&rot);
        RotAndTrans {
            rot,
            trans
        }
    }
}

fn combine_rt(first: &RotAndTrans, second: &RotAndTrans) -> RotAndTrans {
    RotAndTrans {
        rot: second.rot * first.rot,
        trans: first.trans.apply_rot(&second.rot) + second.trans,
    }
}

fn try_find_trans(socket: &HashSet<Vec3i>, plug: &[Vec3i]) -> Option<Vec3i> {
    for sock_vec in socket.iter() {
        for j in 0..plug.len() {
            // try to pair i and j
            let d = *sock_vec - plug[j];
            let mut new_ls = socket.clone();
            new_ls.extend(plug.iter().map(|v| *v + d));
            if (new_ls.len() + 12) <= (socket.len() + plug.len()) {
                return Some(d);
            }
        }
    }
    return None;
}

fn try_find_rt(socket: &HashSet<Vec3i>, plug: &HashSet<Vec3i>) -> Option<RotAndTrans> {
    for rot in ALL_ROTATIONS.iter().copied() {
        let rot_plug = plug
            .iter().copied()
            .map(|v| v.apply_rot(&rot))
            .collect::<Vec<_>>();
        if let Some(trans) = try_find_trans(socket, rot_plug.as_slice()) {
            return Some(RotAndTrans {
                rot,
                trans,
            });
        }
    }
    None
}

fn try_remove<A, B>(
    ls: &mut Vec<A>, pos: usize,
    mut blank: A, f: impl FnOnce(A) -> Result<B, A>
) -> Option<B> {
    swap(&mut blank, &mut ls[pos]);
    match f(blank) {
        Ok(v) => {
            ls.remove(pos);
            Some(v)
        },
        Err(mut blank) => {
            swap(&mut blank, &mut ls[pos]);
            None
        }
    }
}

fn p1(data: &[FreeChunk]) {
    let mut complete_todo = vec![data[0].clone().place_origin()];
    let mut data = Vec::from(&data[1..]);
    let mut complete_done = vec![];

    loop {
        println!("LEFT: {}", data.len());
        if data.len() == 0 {
            break
        } else {
            match complete_todo.pop() {
                None => panic!("unfulfilled"),
                Some(pc) => {
                    let mut i = 0;
                    while i < data.len() {
                        match try_remove(
                            &mut data, i,
                            FreeChunk::blank(),
                            |v| v.attempt_place_by(&pc)
                        ) {
                            Some(new_p) => {
                                println!("::: {} -> {} == {:?}", pc.2, new_p.2, new_p.1);
                                complete_todo.push(new_p)
                            },
                            None => {
                                i += 1
                            }
                        }
                    }
                    complete_done.push(pc)
                }
            }
        }
    }

    complete_done.extend(complete_todo.into_iter());
    let scanners = complete_done;

    println!("1> {}", combine_placed_chunks(scanners.iter()).len());

    let mut max = 0;
    for i in 0..scanners.len() {
        let i_pos = Vec3i::new(0, 0, 0).apply_rt(&scanners[i].1);
        for j in (i + 1)..scanners.len() {
            let j_pos = Vec3i::new(0, 0, 0).apply_rt(&scanners[j].1);
            let v = i_pos.man_dist(&j_pos);
            if v > max {
                max = v;
            }
        }
    }

    println!("2> {}", max)
}

fn main() {
    let data = read_data();

    p1(data.as_slice())
}
