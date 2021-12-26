#![feature(generic_const_exprs)]

use std::fs::File;
use std::io::Read;
use std::mem::swap;
use std::ops::{Add, Sub};

const FILENAME: &'static str = "i1.txt";

fn read_data() -> Vec<i32> {
    let mut file = File::open(FILENAME).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    data.trim_end().split(',').map(|s| s.parse().unwrap()).collect()
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Matrix<const R: usize, const C: usize> where [(); R * C]: Sized {
    backing: [i128; R * C]
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
    fn ident() -> Self {
        let mut backing = [0; RC * RC];
        for i in 0..RC {
            backing[i * (RC + 1)] = 1;
        }
        Matrix {
            backing
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

fn matrix_pow<const A: usize>(m: &Matrix<A, A>, e: usize) -> Matrix<A, A> where [(); A * A]: Sized {
    if e == 0 {
        Matrix::ident()
    } else if e == 1 {
        m.clone()
    } else {
        let mut sub = matrix_pow(m, e >> 1);
        sub = matrix_multiply(&sub, &sub);
        if (e & 1) != 0 {
            sub = matrix_multiply(&sub, m)
        }
        sub
    }
}

const ONE_SIM: Matrix<9, 9> = Matrix {
    backing: [
        0, 1, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 1, 0, 0,
        1, 0, 0, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0
    ]
};

fn sim_for(data: &mut [i128; 9], time: usize) {
    let m_data = Matrix::<9, 1> {
        backing: *data
    };
    let mut res = matrix_multiply(&matrix_pow(&ONE_SIM, time), &m_data);
    swap(data, &mut res.backing);
}

fn sum_data(data: &[i128; 9]) -> i128 {
    data.iter().copied().map(|v| v as i128).sum::<i128>()
}

fn main() {
    let mut data = [0; 9];
    for n in read_data().iter().copied() {
        data[n as usize] += 1;
    }
    sim_for(&mut data, 80);
    println!("1> {}", sum_data(&data));
    sim_for(&mut data, 176);
    println!("2> {}", sum_data(&data));
}
