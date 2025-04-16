use std::fs;

use anyhow::{anyhow, Context, Ok, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("src/bin/d3a1/input")?;

    let joined = input.lines()
        .collect::<String>();

    let do_lines = joined
        .split("do()")
        .filter_map(|do_line| do_line.split("don't()").next());

    let result = do_lines
        .map(mul_sum)
        .sum::<i32>();

    fs::write("src/bin/d3a2/output", format!("{result}"))?;

    Ok(())
}

fn mul_sum(input: &str) -> i32 {
    let mul_bodies = input
        .lines()
        .flat_map(|line| line.split("mul(").skip(1))
        .filter_map(|mul| mul.split(")").next());
    
    let result = mul_bodies
        .map(|b| {
            let mut iter = b.split(",");
            let v1 = iter.next().context("")?.parse::<i32>()?;
            let v2 = iter.next().context("")?.parse::<i32>()?;
            if iter.next().is_some() {
                return Err(anyhow!(""))
            }
            Ok(v1 * v2)
        })
        .filter_map(Result::ok)
        .sum::<i32>();
    
    result
}