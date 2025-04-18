use std::{fs, ops::Div};

use anyhow::{Context, Ok, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    let (rules, updates) = aoc24::d5::rules_and_updates(input)?;

    let mut sum = 0;
    
    'updates: for update in updates {
        'rules: for (fst, snd) in &rules {
            let ix_fst = match update.iter().position(|p| p.eq(fst)) {
                Some(x) => x,
                None => continue 'rules,
            };
            let ix_snd = match update.iter().position(|p| p.eq(snd)) {
                Some(x) => x,
                None => continue 'rules,
            };
            if ix_fst > ix_snd {
                continue 'updates
            }
        }
        sum += update.get(update.len().div(2)).context("")?;
    }

    fs::write("output", format!("{sum}"))?;

    Ok(())
}