use std::{collections::{HashMap, HashSet}, fs};

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    let width = input.find('\n').unwrap() as i32;
    let height = input.len() as i32 / width;
    
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, cell) in row.char_indices() {
            if cell == '.' {
                continue;
            }
            let x = x as i32;
            let y = y as i32;
            match antennas.get_mut(&cell) {
                Some(locations) => locations.push((x, y)),
                None => { antennas.insert(cell, vec![(x, y)]); },
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_, locations) in antennas {
        for &l1 in locations.iter() {
            for &l2 in locations.iter() {
                if l1 == l2 {
                    continue;
                }
                let i_l1_l2 = l2.0 - l1.0;
                let j_l1_l2 = l2.1 - l1.1;
                let l_an1 = (l2.0 + i_l1_l2, l2.1 + j_l1_l2);
                let l_an2 = (l1.0 - i_l1_l2, l1.1 - j_l1_l2);
                for l_an in [l_an1, l_an2] {
                    if l_an.0 < 0 || l_an.0 >= width || l_an.1 < 0 || l_an.1 >= height {
                        continue;
                    }
                    antinodes.insert(l_an);
                }
            }
        }
    }

    let result = antinodes.len();
    fs::write("output", format!("{result}"))?;

    return Ok(());
}