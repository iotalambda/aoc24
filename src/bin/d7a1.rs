use std::{collections::{HashMap, HashSet}, fs};

use anyhow::{Context, Ok, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    let equations: Vec<(Vec<u64>, u64)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let rhs = parts.next().with_context(|| format!("{} {}: {}", stringify!(line), stringify!(rhs), line))?;
            let rhs = rhs.parse::<u64>()?;
            let lhs = parts.next().with_context(|| format!("{} {}: {}", stringify!(line), stringify!(lhs), line))?;
            let lhs = lhs.split(" ").map(|number| Ok(number.parse::<u64>()?)).collect::<Result<_>>()?;
            Ok((lhs, rhs))
        })
        .collect::<Result<_>>()?;

    let operator_maps = equations
        .iter()
        .map(|(lhs, _)| lhs.len())
        .collect::<HashSet<_>>()
        .iter()
        .map(|&len| {
            let mut map = vec![vec![OPERATORS[0]; len-1]; 1 << (len-1)];
            for (row_ix, row) in map.iter_mut().enumerate() {
                for (col_ix, col) in row.iter_mut().enumerate() {
                    *col = OPERATORS[row_ix / (1 << col_ix) % OPERATORS.len()];
                }
            }
            (len, map)
        })
        .collect::<HashMap<_, _>>();

    let mut total_calibration_result = 0;
    'equations: for (lhs, rhs_expected) in equations {
        let lhs_len = lhs.len();
        let operator_map = operator_maps.get(&lhs_len).with_context(|| format!("no {} with {}={}", stringify!(operator_map), stringify!(lhs_len), lhs_len))?;
        for operators in operator_map {
            let mut lhs = lhs.iter();
            let mut rhs_actual = *lhs.next().with_context(|| format!("no 0th value in {}", stringify!(lhs)))?;
            for operator in operators {
                let lhs = lhs.next().with_context(|| format!("no nth value in {}", stringify!(lhs)))?;
                match operator {
                    Operator::Add => rhs_actual += lhs,
                    Operator::Mul => rhs_actual *= lhs,
                }
            }
            if rhs_actual.eq(&rhs_expected) {
                total_calibration_result += rhs_actual;
                continue 'equations;
            }
        }
    }

    fs::write("output", format!("{total_calibration_result}"))?;
    Ok(())
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Mul,
}

const OPERATORS: [Operator; 2] = [
    Operator::Add,
    Operator::Mul,
];