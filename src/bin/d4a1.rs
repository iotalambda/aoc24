use anyhow::{anyhow, Context, Ok, Result};
use std::fs;
use once_cell::sync::Lazy;

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut n_matches = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            n_matches += count_matches_at(&input, "XMAS", x, y);
        }
    }

    fs::write("output", format!("{n_matches}"))?;

    Ok(())
}

static PERMS: Lazy<Vec<(isize, isize)>> = Lazy::new(|| {
    let offsets = [-1, 0, 1];
    let perms = offsets
        .iter()
        .flat_map(|a| offsets.map(|b| (*a, b)))
        .filter(|(a, b)| a.ne(&0) || b.ne(&0))
        .collect::<Vec<_>>();
    perms
});

fn count_matches_at(input: &Vec<Vec<char>>, v: &str, x: usize, y: usize) -> u32 {
    fn is_match(input: &Vec<Vec<char>>, v: &str, x_0: usize, y_0: usize, next_xy: impl Fn(usize, usize) -> (usize, usize)) -> Result<()> {
        let mut x = x_0;
        let mut y = y_0;
        for c_exp in v.chars() {
            let c_act = input.get(y).context("")?.get(x).context("")?;
            if c_act.ne(&c_exp) {
                return Err(anyhow!(""));
            }
            (x, y) = next_xy(x, y);
        }
        Ok(())
    }

    let n_matches: u32 = PERMS
        .iter()
        .map(|(a, b)| is_match(input, v, x, y, |x, y| (x.wrapping_add_signed(*a), y.wrapping_add_signed(*b))))
        .filter_map(Result::ok)
        .count()
        .try_into()
        .expect("Could not convert to u32.");

    n_matches
}