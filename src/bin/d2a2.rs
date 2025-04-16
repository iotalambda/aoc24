use anyhow::{Ok, Result, Context};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("src/bin/d2a1/input")?;
    let mut records = Vec::<Vec<i32>>::new();
    for line in input.lines() {
        let mut levels = Vec::<i32>::new();
        for level in line.split_ascii_whitespace().map(|v| v.parse::<i32>().context("Bad number.")).collect::<Result<Vec<_>>>()? {
            levels.push(level);
        }
        records.push(levels);
    }

    let mut safe = 0;
    'record: for record in records {
        if let Some(uix) = unsafe_ix(record.iter()) {
            'check: {
                if unsafe_ix(without_ix(&record, uix)) == None {
                    break 'check
                } else if uix > 0 && unsafe_ix(without_ix(&record, uix - 1)) == None {
                    break 'check
                } else if uix < record.len() && unsafe_ix(without_ix(&record, uix + 1)) == None {
                    break 'check
                }
                continue 'record
            }
        }

        safe += 1;
    }

    fs::write("src/bin/d2a2/output", format!("{safe}"))?;

    Ok(())
}

fn without_ix<T>(v: &[T], ix: usize) -> impl Iterator<Item = &T> {
    v[..ix].iter().chain(&v[ix+1..])
}

fn unsafe_ix<'a>(iter: impl Iterator<Item = &'a i32>) -> Option<usize> {
    #[derive(PartialEq)]
    enum Dir { Incr, Decr, }
    let mut dir: Option<Dir> = None;
    let mut first = true;
    let mut l_ix: usize = 0;
    let mut val_prev = 0;
    for val in iter {
        let diff = val_prev - val;
        val_prev = *val;

        if first {
            first = false;
            continue
        }

        let dir_curr = match diff {
            v if v.abs() > 3 => return Some(l_ix),
            v if v > 0 => Dir::Incr,
            v if v < 0 => Dir::Decr,
            _ => return Some(l_ix),
        };

        dir = match dir {
            None => Some(dir_curr),
            Some(v) if v != dir_curr => return Some(l_ix),
            _ => dir
        };

        l_ix += 1;
    }
    None
}