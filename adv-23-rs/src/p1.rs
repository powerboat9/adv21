use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::collections::btree_map::Entry;
use std::fmt::Arguments;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::iter::from_fn;
use std::mem::swap;
use crate::common::Color;

fn hall_idx_to_pos(idx: usize) -> u64 {
    [0, 1, 3, 5, 7, 9, 10][idx]
}

fn room_idx_to_pos(idx: usize) -> u64 {
    [2, 4, 6, 8, 2, 4, 6, 8][idx]
}

fn room_idx_to_off(idx: usize) -> u64 {
    [1, 1, 1, 1, 2, 2, 2, 2][idx]
}

fn get_path_score(hall_idx: usize, room_idx: usize) -> u64 {
    hall_idx_to_pos(hall_idx).abs_diff(room_idx_to_pos(room_idx)) + room_idx_to_off(room_idx)
}

fn get_room_take_idx(rooms: &[Option<Color>; 8], r_idx_min: usize) -> Option<usize> {
    if rooms[r_idx_min].is_some() {
        None
    } else if rooms[r_idx_min | 4].is_none() {
        Some(r_idx_min | 4)
    } else if rooms[r_idx_min | 4] != Some(Color::from_idx(r_idx_min)) {
        None
    } else {
        Some(r_idx_min)
    }
}

fn get_room_give_idx(rooms: &[Option<Color>; 8], r_idx_min: usize) -> Option<usize> {
    let target = Color::from_idx(r_idx_min);
    if rooms[r_idx_min].is_some() {
        if (rooms[r_idx_min] == Some(target)) && (rooms[r_idx_min | 4] == Some(target)) {
            None
        } else {
            Some(r_idx_min)
        }
    } else {
        if rooms[r_idx_min | 4].is_some() && (rooms[r_idx_min | 4] != Some(target)) {
            Some(r_idx_min | 4)
        } else {
            None
        }
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

fn best_score_r(mut hall: [Option<Color>; 7], mut rooms: [Option<Color>; 8], score: u64, min: &mut Option<u64>, hist: Vec<(usize, usize, u64, Color)>, best_hist: &mut Option<Vec<(usize, usize, u64, Color)>>) {
    if let Some(min) = min {
        if *min <= score {
            return
        }
    }
    // check for win
    if rooms == [
        Some(Color::Amber), Some(Color::Bronze), Some(Color::Copper), Some(Color::Desert),
        Some(Color::Amber), Some(Color::Bronze), Some(Color::Copper), Some(Color::Desert)
    ] {
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

pub(crate) fn best_score(hall: [Option<Color>; 7], rooms: [Option<Color>; 8]) -> u64 {
    let mut min = None;
    let mut best = None;
    best_score_r(hall, rooms, 0, &mut min, Vec::new(), &mut best);
    let best = best.unwrap();
    println!("?? {:?}", &best);
    let mut hall = hall;
    let mut rooms = rooms;
    let mut i = 0;
    loop {
        display_game(&hall, &rooms);
        if i >= best.len() {
            break
        }
        println!("{}: r[{}] <-> h[{}] == {}, {:?}", i, best[i].0, best[i].1, best[i].2, best[i].3);
        swap(&mut rooms[best[i].0], &mut hall[best[i].1]);
        i += 1
    }
    min.unwrap()
}
