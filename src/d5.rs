use anyhow::{Ok, Result};

pub fn rules_and_updates(input: String) -> Result<(Vec<(u32, u32)>, Vec<Vec<u32>>)> {
    let mut input = input.lines();

    let mut rules = vec![];
    loop {
        let line = input.next().unwrap();
        if line.is_empty() { break }
        let mut line = line.split("|");
        let fst = line.next().unwrap().parse::<u32>()?;
        let snd = line.next().unwrap().parse::<u32>()?;
        rules.push((fst, snd));
    }

    let mut updates = vec![];
    loop {
        let line = match input.next() {
            Some(line) => line,
            None => break,
        };
        let line = line.split(',').map(|p| p.parse::<u32>()).collect::<Result<_, _>>()?;
        updates.push(line);
    }

    Ok((rules, updates))
}