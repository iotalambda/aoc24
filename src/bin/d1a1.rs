use anyhow::{Ok, Result, Context};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("src/bin/d1a1/input")?;

    let mut col1 = Vec::<u32>::new();
    let mut col2 = Vec::<u32>::new();

    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        col1.push(parts.next().context("Missing first col")?.parse()?);
        col2.push(parts.next().context("Missing second col")?.parse()?);
    }

    col1.sort();
    col2.sort();

    let result = col1.iter()
        .zip(col2.iter())
        .map(|(v1, v2)| (*v1).abs_diff(*v2))
        .sum::<u32>();

    fs::write("src/bin/d1a1/output", format!("{result}"))?;

    Ok(())
}