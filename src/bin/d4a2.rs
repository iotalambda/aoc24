use anyhow::{anyhow, Context, Ok, Result};
use std::{fs, ops::Mul};
use once_cell::sync::Lazy;

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut n_xmas = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if count_matches_at(&input, "MAS", x, y) == 2 {
                n_xmas += 1;
            }
        }
    }

    fs::write("output", format!("{n_xmas}"))?;

    Ok(())
}

const PERMS: Lazy<Vec<(isize, isize)>> = Lazy::new(|| {
    let offsets = [-1, 1];
    let perms = offsets
        .iter()
        .flat_map(|a| offsets.map(|b| (*a, b)))
        .collect::<Vec<_>>();
    perms
});

fn count_matches_at(input: &Vec<Vec<char>>, v: &str, x: usize, y: usize) -> u32 {
    fn is_match(
        input: &Vec<Vec<char>>,
        v: &str, x_0: usize, y_0: usize,
        prev_xy: impl Fn(usize, usize) -> (usize, usize),
        next_xy: impl Fn(usize, usize) -> (usize, usize)) -> Result<()> {
            let (mut x, mut y) = prev_xy(x_0, y_0);
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
        .map(|(a, b)| is_match(input, v, x, y, |x, y| (x.wrapping_add_signed(a.mul(-1)), y.wrapping_add_signed(b.mul(-1))), |x, y| (x.wrapping_add_signed(*a), y.wrapping_add_signed(*b))))
        .filter_map(Result::ok)
        .count()
        .try_into()
        .expect("Could not convert to u32.");

    n_matches
}