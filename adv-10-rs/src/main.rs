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

fn read_data() -> Vec<Vec<(PType, Action)>> {
    read_lines().map(|v| v.chars().filter_map(|c| {
        Some(match c {
            '(' => (PType::Paren, Action::Open),
            '[' => (PType::Square, Action::Open),
            '{' => (PType::Curly, Action::Open),
            '<' => (PType::Arrow, Action::Open),
            ')' => (PType::Paren, Action::Close),
            ']' => (PType::Square, Action::Close),
            '}' => (PType::Curly, Action::Close),
            '>' => (PType::Arrow, Action::Close),
            _ => return None
        })
    }).collect()).collect()
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum PType {
    Paren,
    Square,
    Curly,
    Arrow
}

#[derive(Copy, Clone)]
enum Action {
    Open,
    Close
}

fn main() {
    let data = read_data();

    let mut res_1 = 0;
    let mut line_scores = Vec::new();
    'line_it: for line in data.iter() {
        let mut queue = Vec::new();
        for ent in line.iter() {
            match ent.1 {
                Action::Open => {
                    queue.push(ent.0)
                },
                Action::Close => {
                    if queue.pop() != Some(ent.0) {
                        res_1 += match ent.0 {
                            PType::Paren => 3,
                            PType::Square => 57,
                            PType::Curly => 1197,
                            PType::Arrow => 25137
                        };
                        continue 'line_it;
                    }
                }
            }
        }
        let mut score = 0usize;
        while let Some(c) = queue.pop() {
            score *= 5;
            score += match c {
                PType::Paren => 1,
                PType::Square => 2,
                PType::Curly => 3,
                PType::Arrow => 4
            };
        }
        line_scores.push(score);
    }
    line_scores.sort();
    println!("1> {}", res_1);
    println!("2> {}", line_scores[line_scores.len() / 2]);
}
