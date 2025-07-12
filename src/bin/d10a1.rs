use std::fs;

use anyhow::{Context, Ok, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    let map = input
        .lines()
        .map(|line| line
            .trim()
            .chars()
            .map(|char| char.to_digit(10).with_context(|| format!("Could not parse char {char} to digit")).map(|digit| digit as usize))
            .collect())
        .collect::<Result<Vec<Vec<_>>>>()?;
    let y_max = map.len();
    let x_max = map.get(0).unwrap().len();
    let mut history: Vec<Vec<u32>> = vec![vec![0; x_max]; y_max];
    let mut reachable_nines: Vec<Vec<u32>> = vec![vec![0; x_max]; y_max];
    let mut nine_counter = 1;
    for (y, row) in map.iter().enumerate() {
        for (x, &h) in row.iter().enumerate() {
            if h == 9 {
                dfs(&mut history, h, x, y, &mut reachable_nines, &map, nine_counter);
                nine_counter += 1;
            }
        }
    }
    let mut score = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, &h) in row.iter().enumerate() {
            if h == 0 {
                score += reachable_nines[y][x];
            }
        }
    }
    fs::write("output", score.to_string())?;
    Ok(())
}


fn dfs(history: &mut Vec<Vec<u32>>, h: usize, x: usize, y: usize, reachable_nines: &mut Vec<Vec<u32>>, map: &Vec<Vec<usize>>, nine_counter: u32) {
    if h == 0 {
        return;
    }
    for &(d_x, d_y) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let x_2 = x.wrapping_add_signed(d_x);
        let y_2 = y.wrapping_add_signed(d_y);
        if let Some(&h_2) = map.get(y_2).and_then(|row| row.get(x_2)) {
            if h_2 == h - 1 && history[y_2][x_2] != nine_counter {
                history[y_2][x_2] = nine_counter;
                reachable_nines[y_2][x_2] += 1;
                dfs(history, h_2, x_2, y_2, reachable_nines, map, nine_counter);
            }
        }
    }
}
