use std::fs;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    let (rules, updates) = aoc24::d5::rules_and_updates(input)?;

    let matching_rules_max = updates.iter().map(|u| u.len()).max().unwrap().pow(2);
    let mut matching_rules: Vec<&(u32, u32)> = Vec::with_capacity(matching_rules_max);
    let mut matching_rules_normalized: Vec<&(u32, u32)> = Vec::with_capacity(matching_rules_max);

    let mut sum = 0;

    for update in updates {

        matching_rules.clear();
        matching_rules_normalized.clear();

        matching_rules.extend(rules
            .iter()
            .filter(|(fst, snd)| update.contains(fst) && update.contains(snd)));

        for ix_0 in 0..matching_rules.len() {
            
            let ix_1 = matching_rules
                .get(ix_0.wrapping_sub(1))
                .and_then(|(prev_fst, _)| matching_rules
                    .iter()
                    .skip(ix_0)
                    .position(|(fst, _)| fst.eq(prev_fst)))
                .map(|ix| ix_0 + ix);

            let ix_1 = ix_1.or_else(|| matching_rules
                .iter()
                .skip(ix_0)
                .position(|(fst_0, _)| matching_rules
                    .iter()
                    .skip(ix_0)
                    .all(|(_, snd_1)| fst_0.ne(snd_1)))
                .map(|ix| ix_0 + ix));
            
            let ix_1 = ix_1.unwrap();

            matching_rules.swap(ix_0, ix_1);
        }

        matching_rules_normalized.extend(matching_rules
            .iter()
            .enumerate()
            .filter(|(ix, (fst_0, snd_0))| matching_rules
                .iter()
                .skip(ix + 1)
                .take_while(|(fst_1, _)| snd_0.ne(fst_1))
                .all(|(fst_1, _)| fst_0.eq(fst_1)))
            .map(|(_, rule)| rule));

        let update_sorted = || matching_rules_normalized
            .iter()
            .map(|(fst, _)| fst)
            .chain(matching_rules_normalized
                .last()
                .map(|(_, snd)| snd));

        if update.iter().ne(update_sorted()) {
            let update_sorted = update_sorted().nth(update.len() / 2).unwrap();
            sum += update_sorted;
        }
    }

    fs::write("output", format!("{sum}"))?;

    Ok(())
}