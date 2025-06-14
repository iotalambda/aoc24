use std::{collections::{HashMap, HashSet}, fs};

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    let width = input.find('\n').unwrap() as i32 ;
    let height = input.len() as i32 / width;

    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    struct Location { x: i32, y: i32 }

    let mut antennas: HashMap<char, Vec<Location>> = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, cell) in row.char_indices() {
            if cell == '.' {
                continue;
            }
            let loc = Location{ x: x as i32, y: y as i32 };
            match antennas.get_mut(&cell) {
                Some(locations) => locations.push(loc),
                None => { antennas.insert(cell, vec![loc]); },
            }
        }
    }

    let mut antinodes: HashSet<Location> = HashSet::new();
    for (_, locations) in antennas {
        for (ix_l1, &l1) in locations.iter().enumerate() {
            for &l2 in locations.iter().skip(ix_l1 + 1) {
                let i_l1_l2 = l2.x - l1.x;
                let j_l1_l2 = l2.y - l1.y;
                for dir in [1, -1] {
                    for mul in 0.. {
                        let l_an = Location{ x: l2.x + dir * mul * i_l1_l2, y: l2.y + dir * mul * j_l1_l2 };
                        if l_an.x < 0 || l_an.x >= width || l_an.y < 0 || l_an.y >= height {
                            break;
                        }
                        antinodes.insert(l_an);
                    }
                }
            }
        }
    }

    let result = antinodes.len();
    fs::write("output", format!("{result}"))?;

    return Ok(());
}