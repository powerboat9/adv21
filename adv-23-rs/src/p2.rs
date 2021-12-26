use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::collections::hash_map::Entry;
use std::fmt::Arguments;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::iter::from_fn;
use std::mem::{MaybeUninit, swap};
use crate::common::Color;

fn hall_idx_to_pos(idx: usize) -> u64 {
    [0, 1, 3, 5, 7, 9, 10][idx]
}

fn room_idx_to_pos(idx: usize) -> u64 {
    [2, 4, 6, 8, 2, 4, 6, 8, 2, 4, 6, 8, 2, 4, 6, 8][idx]
}

fn room_idx_to_off(idx: usize) -> u64 {
    [1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4][idx]
}

fn get_path_score(hall_idx: usize, room_idx: usize) -> u64 {
    hall_idx_to_pos(hall_idx).abs_diff(room_idx_to_pos(room_idx)) + room_idx_to_off(room_idx)
}

fn get_room_take_idx(rooms: &[Option<Color>; 16], r_idx_min: usize) -> Option<usize> {
    let target = Color::from_idx(r_idx_min);
    // verify perfect/empty
    for i in (0..4).rev() {
        let r_idx = i * 4 + r_idx_min;
        match rooms[r_idx] {
            None => return Some(r_idx),
            Some(t) if t != target => return None,
            _ => {}
        }
    }
    None
}

fn get_room_give_idx(rooms: &[Option<Color>; 16], r_idx_min: usize) -> Option<usize> {
    let target = Color::from_idx(r_idx_min);
    // check if empty/perfect
    let mut had_good = false;
    for i in (0..4).rev() {
        let r_idx = i * 4 + r_idx_min;
        match rooms[r_idx] {
            None => {
                return if had_good {
                    Some(r_idx + 4)
                } else {
                    None
                }
            },
            Some(c) => {
                if c != target {
                    had_good = true;
                }
            }
        }
    }
    if had_good {
        Some(r_idx_min)
    } else {
        None
    }
}

fn left_search_hall(hall: &[Option<Color>; 7], start_idx: usize, target: Color) -> Option<usize> {
    for h_idx in (0..=start_idx).rev() {
        if let Some(h_color) = hall[h_idx] {
            return if h_color == target {
                Some(h_idx)
            } else {
                None
            }
        }
    }
    None
}

fn right_search_hall(hall: &[Option<Color>; 7], start_idx: usize, target: Color) -> Option<usize> {
    for h_idx in start_idx..7 {
        if let Some(h_color) = hall[h_idx] {
            return if h_color == target {
                Some(h_idx)
            } else {
                None
            }
        }
    }
    None
}

fn left_find_empty_hall<'a>(hall: &'a [Option<Color>; 7], start_idx: usize) -> impl 'a + Iterator<Item=usize> {
    (0..=start_idx)
        .rev()
        .take_while(|h_idx| {
            hall[*h_idx].is_none()
        })
}

fn right_find_empty_hall<'a>(hall: &'a [Option<Color>; 7], start_idx: usize) -> impl 'a + Iterator<Item=usize> {
    (start_idx..7)
        .take_while(|h_idx| {
            hall[*h_idx].is_none()
        })
}

fn find_empty_hall_around<'a>(hall: &'a [Option<Color>; 7], r_idx_min: usize) -> impl 'a + Iterator<Item=usize> {
    left_find_empty_hall(hall, r_idx_min + 1)
        .chain(right_find_empty_hall(hall, r_idx_min + 2))
}

fn search_hall_around(hall: &[Option<Color>; 7], r_idx_min: usize) -> Option<usize> {
    let target = Color::from_idx(r_idx_min);
    if let Some(r) = left_search_hall(hall, r_idx_min + 1, target) {
        Some(r)
    } else if let Some(r) = right_search_hall(hall, r_idx_min + 2, target) {
        Some(r)
    } else {
        None
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Game {
    hall: [Option<Color>; 7],
    rooms: [Option<Color>; 16],
    score: u64,
    moves: u32
    //hist: Vec<(usize, usize, u64, Color)>
}

impl PartialOrd<Self> for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        self.moves.cmp(&other.moves)
    }
}

impl Game {
    fn unique_id(&self) -> u64 {
        let mut acc = 0;
        self.hall.iter().chain(self.rooms.iter())
            .for_each(|v| {
                acc *= 5;
                acc += match v {
                    Some(c) => c.get_idx() as u64,
                    None => 4
                }
            });
        acc
    }

    fn get_take_to_idx(&self, r_idx_min: usize) -> Option<usize> {
        for i in (0..4).rev() {
            let r_idx = i * 4 + r_idx_min;
            match self.rooms[r_idx] {
                None => return Some(r_idx),
                Some(c) => {
                    if c.get_idx() != r_idx_min {
                        return None
                    }
                }
            }
        }
        None
    }

    fn get_give_from_idx(&self, r_idx_min: usize) -> Option<usize> {
        let mut ret = None;
        for i in (0..4).rev() {
            let r_idx = i * 4 + r_idx_min;
            match self.rooms[r_idx] {
                None => break,
                Some(c) => {
                    if ret.is_some() || (c.get_idx() != r_idx_min) {
                        ret = Some(r_idx)
                    }
                }
            }
        }
        ret
    }

    fn is_winning(&self) -> bool {
        let r = self.rooms
            .chunks_exact(4)
            .map(|c| {
                c == [Some(Color::Amber), Some(Color::Bronze), Some(Color::Copper), Some(Color::Desert)]
            })
            .all(|v| v);
        if r {
            println!("WIN: {}", self.score);
        }
        r
    }

    fn has_room_jam(&self) -> bool {
        // checks for room that is unable to clear itself
        false
    }

    fn is_path_between_rooms(&self, idx_low_min: usize, idx_high_min: usize) -> bool {
        for hall_idx in (idx_low_min+2)..=(idx_high_min+1) {
            if self.hall[hall_idx].is_some() {
                return false
            }
        }
        true
    }

    fn room_path_cost(idx_low: usize, idx_high: usize) -> u64 {
        room_idx_to_pos(idx_high)
            - room_idx_to_pos(idx_low)
            + room_idx_to_off(idx_low)
            + room_idx_to_off(idx_high)
    }

    fn with_swap_hr(mut self, hall_idx: usize, room_idx: usize, c: Color) -> Self {
        swap(&mut self.hall[hall_idx], &mut self.rooms[room_idx]);
        if self.hall == [None, None, None, None, None, None, Some(Color::Desert)] {
            println!("BASED");
        }
        self.score += get_path_score(hall_idx, room_idx) * c.get_cost();
        self.moves += 1;
        self
    }

    fn with_swap_rr(mut self, i1: usize, i2: usize, c: Color) -> Self {
        let idx_low = i1.min(i2);
        let idx_high = i1.max(i2);
        let (a, b) = self.rooms.split_at_mut(idx_high);
        let a = &mut a[idx_low];
        let b = &mut b[0];
        swap(a, b);
        self.score += Game::room_path_cost(idx_low, idx_high) * c.get_cost();
        self.moves += 1;
        self
    }

    fn reproduce(self, world_in: &mut World) {
        if self.hall == [None, None, None, None, None, None, Some(Color::Desert)] {
            println!("BASED 2");
        }
        if world_in.min_score <= self.score {
            println!("DITCH");
            return
        }
        // check for win
        if self.is_winning() {
            println!("W: {}", self.score);
            world_in.min_score = self.score;
            //world_in.best_hist = self.hist;
            return
        }
        // rooms steal from hall
        // always best move, has to happen eventually, clears hall
        for r_idx_min in 0..4 {
            let target = Color::from_idx(r_idx_min);
            let r_idx = if let Some(v) = self.get_take_to_idx(r_idx_min) {
                v
            } else {
                continue
            };
            // search for target in hallway
            if let Some(h_idx) = search_hall_around(&self.hall, r_idx_min) {
                world_in.add_game(self.with_swap_hr(h_idx, r_idx, target));
                return
            }
        }
        // rooms punt to rooms
        // also always best move
        /*
        for idx_from_min in 0..4 {
            let idx_from = if let Some(idx_from) = get_room_give_idx(&self.rooms, idx_from_min) {
                idx_from
            } else {
                continue
            };
            let c = self.rooms[idx_from].unwrap();
            let idx_to_min = c.get_idx();
            let idx_to = if idx_to_min == idx_from_min {
                continue
            } else if let Some(idx_to) = get_room_take_idx(&self.rooms, idx_to_min) {
                idx_to
            } else {
                continue
            };
            world_in.add_game(self.with_swap_rr(idx_to, idx_from, c));
            return;
        }
         */
        // rooms punt to hall
        for r_idx_min in 0..4 {
            let r_idx = if let Some(v) = self.get_give_from_idx(r_idx_min) {
                v
            } else {
                continue
            };
            let move_color = self.rooms[r_idx].unwrap();
            // search for empty slots in hallway
            for h_idx in find_empty_hall_around(&self.hall, r_idx_min) {
                if (h_idx == 6) && (r_idx == 3) {
                }
                world_in.add_game(self.clone().with_swap_hr(h_idx, r_idx, move_color));
            }
        }
    }
}

struct World {
    games: Vec<Game>,
    min_score: u64,
    //best_hist: Vec<(usize, usize, u64, Color)>,
    min_scores: HashMap<Game, u64>
}

impl World {
    fn new() -> Self {
        World {
            games: Vec::new(),
            min_score: u64::MAX,
            //best_hist: Vec::new(),
            min_scores: HashMap::new()
        }
    }

    fn with_game(mut self, g: Game) -> Self {
        self.add_game(g);
        self
    }

    fn add_game(&mut self, g: Game) {
        if g.hall == [None, None, None, None, None, None, Some(Color::Desert)] {
            println!("BASED 1.5");
        }
        match self.min_scores.entry(g.clone()) {
            Entry::Occupied(mut v) => {
                let v = v.get_mut();
                if *v <= g.score {
                    return;
                }
                *v = g.score
            }
            Entry::Vacant(v) => {
                v.insert(g.score);
            }
        }
        if g.hall == [None, None, None, None, None, None, Some(Color::Desert)] {
            println!("BASED 1.75");
        }
        self.games.push(g);
    }

    fn run(mut self) -> u64 {
        while let Some(g) = self.games.pop() {
            g.reproduce(&mut self);
        }
        self.min_score
    }
}

/*
fn best_score_r(mut hall: [Option<Color>; 7], mut rooms: [Option<Color>; 16], score: u64, min: &mut Option<u64>, hist: Vec<(usize, usize, u64, Color)>, best_hist: &mut Option<Vec<(usize, usize, u64, Color)>>) {
    if let Some(min) = min {
        if *min <= score {
            return
        }
    }
    // check for win
    if rooms == [
        Some(Color::Amber), Some(Color::Bronze), Some(Color::Copper), Some(Color::Desert),
        Some(Color::Amber), Some(Color::Bronze), Some(Color::Copper), Some(Color::Desert),
        Some(Color::Amber), Some(Color::Bronze), Some(Color::Copper), Some(Color::Desert),
        Some(Color::Amber), Some(Color::Bronze), Some(Color::Copper), Some(Color::Desert)
    ] {
        println!("M: {}", score);
        *min = Some(score);
        *best_hist = Some(hist);
        return
    }
    // rooms steal from hall
    // always best move, has to happen eventually, clears hall
    for r_idx_min in 0..4 {
        let target = Color::from_idx(r_idx_min);
        let r_idx = if let Some(v) = get_room_take_idx(&rooms, r_idx_min) {
            v
        } else {
            continue
        };
        // search for target in hallway
        if let Some(h_idx) = search_hall_around(&hall, r_idx_min) {
            hall[h_idx] = None;
            rooms[r_idx] = Some(target);
            let mut hist = hist;
            hist.push((r_idx, h_idx, get_path_score(h_idx, r_idx) * target.get_cost(), target));
            return best_score_r(
                hall, rooms,
                score +
                    get_path_score(h_idx, r_idx) * target.get_cost(),
                min,
                hist,
                best_hist
            )
        }
    }
    // rooms punt to hall
    for r_idx_min in 0..4 {
        let r_idx = if let Some(v) = get_room_give_idx(&rooms, r_idx_min) {
            v
        } else {
            continue
        };
        let target = rooms[r_idx].unwrap();
        // search for empty slots in hallway
        for h_idx in find_empty_hall_around(&hall, r_idx_min) {
            let mut new_hall = hall;
            let mut new_rooms = rooms;
            let mut new_hist = hist.clone();
            new_hist.push((r_idx, h_idx, get_path_score(h_idx, r_idx) * target.get_cost(), target));
            new_hall[h_idx] = new_rooms[r_idx];
            new_rooms[r_idx] = None;
            best_score_r(
                new_hall, new_rooms,
                score +
                    get_path_score(h_idx, r_idx) * target.get_cost(),
                min,
                new_hist,
                best_hist
            )
        }
    }
}
 */

/*
fn display_game(hall: &[Option<Color>; 7], rooms: &[Option<Color>; 8]) {
    println!("#############");
    println!("#{}{}.{}.{}.{}.{}{}#",
             hall[0].map(|v| v.get_letter()).unwrap_or('.'),
             hall[1].map(|v| v.get_letter()).unwrap_or('.'),
             hall[2].map(|v| v.get_letter()).unwrap_or('.'),
             hall[3].map(|v| v.get_letter()).unwrap_or('.'),
             hall[4].map(|v| v.get_letter()).unwrap_or('.'),
             hall[5].map(|v| v.get_letter()).unwrap_or('.'),
             hall[6].map(|v| v.get_letter()).unwrap_or('.')
    );
    println!("###{}#{}#{}#{}###",
             rooms[0].map(|v| v.get_letter()).unwrap_or('.'),
             rooms[1].map(|v| v.get_letter()).unwrap_or('.'),
             rooms[2].map(|v| v.get_letter()).unwrap_or('.'),
             rooms[3].map(|v| v.get_letter()).unwrap_or('.'),
    );
    println!("  #{}#{}#{}#{}#",
             rooms[4].map(|v| v.get_letter()).unwrap_or('.'),
             rooms[5].map(|v| v.get_letter()).unwrap_or('.'),
             rooms[6].map(|v| v.get_letter()).unwrap_or('.'),
             rooms[7].map(|v| v.get_letter()).unwrap_or('.'),
    );
    println!("  #########");
}
 */

pub(crate) fn best_score(hall: [Option<Color>; 7], rooms: [Option<Color>; 16]) -> u64 {
    /*
    let mut min = None;
    let mut best = None;
    best_score_r(hall, rooms, 0, &mut min, Vec::new(), &mut best);
    let best = best.unwrap();
    //println!("?? {:?}", &best);
    let mut hall = hall;
    let mut rooms = rooms;
    let mut i = 0;
    loop {
        //display_game(&hall, &rooms);
        if i >= best.len() {
            break
        }
        //println!("{}: r[{}] <-> h[{}] == {}, {:?}", i, best[i].0, best[i].1, best[i].2, best[i].3);
        swap(&mut rooms[best[i].0], &mut hall[best[i].1]);
        i += 1
    }
    min.unwrap()
     */
    let start = Game {
        hall,
        rooms,
        score: 0,
        moves: 0
    };
    World::new()
        .with_game(start)
        .run()
}
