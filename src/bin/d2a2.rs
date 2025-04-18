use anyhow::{Ok, Result};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;

    let records: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line
            .split_ascii_whitespace()
            .map(|v| v.parse::<i32>())
            .collect::<Result<_, _>>()
        )
        .collect::<Result<_, _>>()?;

    let safe = records
        .iter()
        .filter(|r| is_safe(r))
        .count();

    fs::write("output", format!("{safe}"))?;

    Ok(())
}

fn is_safe(record: &[i32]) -> bool {
    if let Some(uix) = unsafe_ix(record.iter()) {
        for cix in [uix.wrapping_sub(1), uix, uix + 1] {
            if unsafe_ix(without_ix(record, cix)).is_none() {
                return true
            }
        }
        return false
    }
    true
}

fn without_ix<T>(v: &[T], ix: usize) -> impl Iterator<Item = &T> {
    v.iter().enumerate().filter(move |(i, _)| *i != ix).map(|(_, v)| v)
}

fn unsafe_ix<'a>(iter: impl Iterator<Item = &'a i32>) -> Option<usize> {
    
    #[derive(PartialEq)]
    enum Dir { Incr, Decr, }
    
    let mut dir: Option<Dir> = None;
    let mut prev: Option<i32> = None;

    for (ix, &val) in iter.enumerate() {
        if let Some(prev) = prev {
            let ix = ix - 1;
            let diff = val - prev;

            let dir_curr = match diff {
                v if v.abs() > 3 => return Some(ix),
                v if v > 0 => Dir::Incr,
                v if v < 0 => Dir::Decr,
                _ => return Some(ix),
            };

            dir = match dir {
                None => Some(dir_curr),
                Some(dir) if dir != dir_curr => return Some(ix),
                _ => dir,
            };
        }
        prev = Some(val);
    }
    None
}