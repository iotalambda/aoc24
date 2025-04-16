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
    'records: for record in records {
        #[derive(PartialEq)]
        enum Dir { Incr, Decr, }
        let mut dir: Option<Dir> = None;
        for l_ix in 0..record.len()-1 {
            let diff = record[l_ix] - record[l_ix+1];
            let dir_curr = match diff {
                v if v.abs() > 3 => continue 'records,
                v if v > 0 => Dir::Incr,
                v if v < 0 => Dir::Decr,
                _ => continue 'records,
            };
            dir = match dir {
                None => Some(dir_curr),
                Some(v) if v != dir_curr => continue 'records,
                _ => dir
            };
        }
        safe += 1;
    }

    fs::write("src/bin/d2a1/output", format!("{safe}"))?;

    Ok(())
}