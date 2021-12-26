use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::iter::from_fn;
use std::ops::Deref;
use std::str::Chars;

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

struct DieP1 {
    inner: u32,
    roll_cnt: u32
}

impl DieP1 {
    fn roll(&mut self) -> u32 {
        self.roll_cnt += 1;
        self.inner += 1;
        let r = self.inner;
        self.inner %= 100;
        r
    }

    fn new() -> Self {
        DieP1 {
            inner: 0,
            roll_cnt: 0
        }
    }

    fn get_roll_count(&self) -> u32 {
        self.roll_cnt
    }
}

fn parse_start(s: &str) -> u32 {
    s.rsplit_once(' ').unwrap().1.parse().unwrap()
}

fn read_data() -> (u32, u32) {
    let mut it = read_lines();
    let a = parse_start(it.next().unwrap().as_str());
    let b = parse_start(it.next().unwrap().as_str());
    (a, b)
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Game {
    plays: [(u32, u32); 2],
    is_p2_turn: bool
}

struct Player {
    score: u32,
    pos: u32
}

impl Player {
    fn new_with_pos(pos: u32) -> Self {
        Player {
            score: 0,
            pos: pos - 1
        }
    }

    fn turn(&mut self, n: u32) {
        self.pos += n;
        self.pos %= 10;
        self.score += self.pos + 1;
    }

    fn is_winning(&self) -> bool {
        self.score >= 1000
    }
}

fn p1(player_1: u32, player_2: u32) {
    let mut die = DieP1::new();
    let mut players = [
        Player::new_with_pos(player_1),
        Player::new_with_pos(player_2)
    ];

    let mut cur = false;
    loop {
        players[cur as usize].turn(die.roll() + die.roll() + die.roll());
        if players[cur as usize].is_winning() {
            break
        }
        cur = !cur;
    }

    println!("1> {}", players[!cur as usize].score * die.roll_cnt)
}

fn inc_hashmap<T: Eq + Hash>(hm: &mut HashMap<T, usize>, k: T, n: usize) {
    match hm.entry(k) {
        Entry::Occupied(mut o) => {
            *o.get_mut() += n;
        },
        Entry::Vacant(v) => {
            v.insert(n);
        }
    }
}

fn p2(player_1: u32, player_2: u32) {
    fn play_game_roll(mut g: Game, g_cnt: usize, win_counts: &mut [usize; 2], map: &mut HashMap<Game, usize>, roll: u32) {
        let turn_idx = g.is_p2_turn as usize;
        let cur_player = &mut g.plays[turn_idx];
        cur_player.0 += roll;
        cur_player.0 %= 10;
        cur_player.1 += cur_player.0 + 1;
        if cur_player.1 >= 21 {
            win_counts[turn_idx] += g_cnt;
        } else {
            g.is_p2_turn ^= true;
            inc_hashmap(map, g, g_cnt);
        }
    }

    fn play_game(g: Game, g_cnt: usize, win_counts: &mut [usize; 2], map: &mut HashMap<Game, usize>) {
        for (roll, cnt) in [
            (3, 1),
            (4, 3),
            (5, 6),
            (6, 7),
            (7, 6),
            (8, 3),
            (9, 1)
        ] {
            play_game_roll(g, g_cnt * cnt, win_counts, map, roll)
        }
    }

    let mut games_ls = HashMap::new();
    let mut win_counts = [0; 2];
    games_ls.insert(Game {
        plays: [(player_1 - 1, 0), (player_2 - 1, 0)],
        is_p2_turn: false
    }, 1);
    while games_ls.len() != 0 {
        let mut new_games_ls = HashMap::new();
        for (game, cnt) in games_ls.drain() {
            play_game(game, cnt, &mut win_counts, &mut new_games_ls);
        }
        games_ls = new_games_ls;
    }

    println!("2> {}", win_counts.into_iter().max().unwrap())
}

fn main() {
    let (player_one, player_two) = read_data();

    p1(player_one, player_two);
    p2(player_one, player_two);
}
