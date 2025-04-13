use anyhow::{Ok, Result, Context};
use std::{collections::HashMap, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("src/bin/d1a1/input")?;

    let mut col1 = Vec::<u32>::new();
    let mut col2 = HashMap::<u32, u32>::new();

    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        col1.push(parts.next().context("Missing first col")?.parse()?);
        let v2 = parts.next().context("Missing second col")?.parse()?;
        col2.entry(v2)
            .and_modify(|v2| *v2 += 1)
            .or_insert(1);
    }

    let result = col1.iter()
        .map(|v1| *v1 * col2.get(v1).unwrap_or(&0))
        .sum::<u32>();

    fs::write("src/bin/d1a2/output", format!("{result}"))?;

    Ok(())
}