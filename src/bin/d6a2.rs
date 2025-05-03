use std::{collections::HashMap, fs};

use anyhow::{Context, Ok, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    let map = input.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let w = map.get(0).context("get map row with ix 0")?.len();
    let h = map.len();

    let mut tps: HashMap<(usize, usize), TP> = HashMap::new();
    let (mut x_curr, mut y_curr) = (0, 0);
    for (y, r) in map.iter().enumerate() {
        for (x, &c) in r.iter().enumerate() {
            if c == '^' {
                (x_curr, y_curr) = (x, y);
                continue;
            }
            if c != '#' {
                continue;
            }
            if y > 0 {
                let tp = tps.entry((x, y - 1)).or_insert_with(|| TP::new(x, y - 1));
                tp.down_left = true;
            }
            if y < h - 1 {
                let tp = tps.entry((x, y + 1)).or_insert_with(|| TP::new(x, y + 1));
                tp.up_right = true;
            }
            if x > 0 {
                let tp = tps.entry((x - 1, y)).or_insert_with(|| TP::new(x - 1, y));
                tp.left_up = true;
            }
            if x < w - 1 {
                let tp = tps.entry((x + 1, y)).or_insert_with(|| TP::new(x + 1, y));
                tp.right_down = true;
            }
        }
    }

    let mut cols = (0..w).map(|_| Vec::<TP>::new()).collect::<Vec<_>>();
    let mut rows = (0..h).map(|_| Vec::<TP>::new()).collect::<Vec<_>>();
    for ((x, y), tp) in tps {
        if tp.down_left || tp.up_right {
            let col = cols.get_mut(x).with_context(|| format!("get mut col with ix {x}"))?;
            let ix = col.iter().enumerate().find_map(|(ix, &tp)| if tp.row > y { Some(ix) } else { None }).unwrap_or(col.len());
            col.insert(ix, tp);
        }
        
        if tp.left_up || tp.right_down {
            let row = rows.get_mut(y).with_context(|| format!("get mut row with ix {y}"))?;
            let ix = row.iter().enumerate().find_map(|(ix, &tp)| if tp.col > x { Some(ix) } else { None }).unwrap_or(row.len());
            row.insert(ix, tp);
        }
    }

    let mut dir_curr = Dir::Up;
    let mut step = 0;
    let mut step_history = vec![];
    let mut count = 0;
    loop {
        let (x_off, y_off): (isize, isize) = match dir_curr {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };

        step_history.push((x_curr, y_curr));
        let x_next = x_curr.wrapping_add_signed(x_off);
        let y_next = y_curr.wrapping_add_signed(y_off);

        let loc_next = map.get(y_next).and_then(|row| row.get(x_next));
        match loc_next {
            Some(&loc) if loc == '#' => {
                dir_curr = match dir_curr {
                    Dir::Up => Dir::Right,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                    Dir::Right => Dir::Down,
                };
                continue;
            },
            Some(_) => {
                if !step_history.contains(&(x_next, y_next)) {
                    let (obs_tp_w_ix, obs_tp_e_ix) = {
                        let obs_row = rows.get_mut(y_next).with_context(|| format!("get mut obs_row with ix {y_next}"))?;
                        let obs_tp_w_ix = obs_row.iter().enumerate().find_map(|(ix, &tp)| if tp.col > x_next - 1 { Some(ix) } else { None }).unwrap_or(obs_row.len());
                        let obs_tp_w = TP{ right_down: true, ..TP::new(x_next - 1, y_next) };
                        obs_row.insert(obs_tp_w_ix, obs_tp_w);
                        let obs_tp_e_ix = obs_row.iter().enumerate().find_map(|(ix, &tp)| if tp.col > x_next + 1 { Some(ix) } else { None }).unwrap_or(obs_row.len());
                        let obs_tp_e = TP{ left_up: true, ..TP::new(x_next + 1, y_next) };
                        obs_row.insert(obs_tp_e_ix, obs_tp_e);
                        (obs_tp_w_ix, obs_tp_e_ix)
                    };

                    let (obs_tp_n_ix, obs_tp_s_ix) = {
                        let obs_col = cols.get_mut(x_next).with_context(|| format!("get mut obs_col with ix {x_next}"))?;
                        let obs_tp_n_ix = obs_col.iter().enumerate().find_map(|(ix, &tp)| if tp.row > y_next - 1 { Some(ix) } else { None }).unwrap_or(obs_col.len());
                        let obs_tp_n = TP{ down_left: true, ..TP::new(x_next, y_next - 1)};
                        obs_col.insert(obs_tp_n_ix, obs_tp_n);
                        let obs_tp_s_ix = obs_col.iter().enumerate().find_map(|(ix, &tp)| if tp.row > y_next + 1 { Some(ix) } else { None }).unwrap_or(obs_col.len());
                        let obs_tp_s = TP{ up_right: true, ..TP::new(x_next, y_next + 1)};
                        obs_col.insert(obs_tp_s_ix, obs_tp_s);
                        (obs_tp_n_ix, obs_tp_s_ix)
                    };

                    let mut dir_curr2 = match dir_curr {
                        Dir::Up => Dir::Right,
                        Dir::Down => Dir::Left,
                        Dir::Left => Dir::Up,
                        Dir::Right => Dir::Down,
                    };
                    let (mut x_curr2, mut y_curr2) = (x_curr, y_curr);
                    loop {
                        let tp_next2 = match dir_curr2 {
                            Dir::Up => cols.get_mut(x_curr2).with_context(|| format!("cols get_mut with ix {x_curr2}"))?.iter_mut().filter(|tp| tp.row < y_curr2).next_back(),
                            Dir::Down => cols.get_mut(x_curr2).with_context(|| format!("cols get_mut with ix {x_curr2}"))?.iter_mut().filter(|tp| tp.row > y_curr2).next(),
                            Dir::Left => rows.get_mut(y_curr2).with_context(|| format!("rows get_mut with ix {y_curr2}"))?.iter_mut().filter(|tp| tp.col < x_curr2).next_back(),
                            Dir::Right => rows.get_mut(y_curr2).with_context(|| format!("rows get_mut with ix {y_curr2}"))?.iter_mut().filter(|tp| tp.col > x_curr2).next(),
                        };

                        match tp_next2 {
                            Some(tp) => {
                                (x_curr2, y_curr2) = (tp.col, tp.row);
                                match dir_curr2 {
                                    Dir::Up => {
                                        if tp.visited_up == step {
                                            count += 1;
                                            break;
                                        }
                                        dir_curr2 = Dir::Right;
                                        tp.visited_up = step;
                                    },
                                    Dir::Down => {
                                        if tp.visited_down == step {
                                            count += 1;
                                            break;
                                        }
                                        dir_curr2 = Dir::Left;
                                        tp.visited_down = step;
                                    },
                                    Dir::Left => {
                                        if tp.visited_left == step {
                                            count += 1;
                                            break;
                                        }
                                        dir_curr2 = Dir::Up;
                                        tp.visited_left = step;
                                    },
                                    Dir::Right => {
                                        if tp.visited_right == step {
                                            count += 1;
                                            break;
                                        }
                                        dir_curr2 = Dir::Down;
                                        tp.visited_right = step;
                                    }
                                }
                            },
                            None => break,
                        }
                    }

                    {
                        let obs_col = cols.get_mut(x_next).with_context(|| format!("get mut obs_col with ix {x_next}"))?;
                        obs_col.remove(obs_tp_s_ix);
                        obs_col.remove(obs_tp_n_ix);
                        let obs_row = rows.get_mut(y_next).with_context(|| format!("get mut obs_row with ix {y_next}"))?;
                        obs_row.remove(obs_tp_e_ix);
                        obs_row.remove(obs_tp_w_ix);
                    }
                }

                (x_curr, y_curr) = (x_next, y_next);
            },
            None => break,
        }

        step += 1;
    }

    fs::write("output", format!("{count}"))?;

    Ok(())
}

#[derive(Clone, Copy)]
struct TP {
    col: usize,
    row: usize,
    left_up: bool,
    down_left: bool,
    right_down: bool,
    up_right: bool,
    visited_up: i32,
    visited_down: i32,
    visited_left: i32,
    visited_right: i32,
}

impl TP {
    fn new(col: usize, row: usize) -> Self {
        Self {
            col, 
            row,
            left_up: false, 
            down_left: false, 
            right_down: false, 
            up_right: false,
            visited_up: -1, 
            visited_down: -1,
            visited_left: -1,
            visited_right: -1,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}