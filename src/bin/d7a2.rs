use std::fs;

use anyhow::{Context, Ok, Result, anyhow};

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    let mut max_lhs = 0;
    let mut max_lhs_length = 0;
    let equations: Vec<(Vec<u64>, u64)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let rhs = parts.next().with_context(|| format!("{} {}: {}", stringify!(line), stringify!(rhs), line))?;
            let rhs = rhs.parse::<u64>()?;
            let lhs = parts.next().with_context(|| format!("{} {}: {}", stringify!(line), stringify!(lhs), line))?;
            let lhs: Vec<_> = lhs.split(" ").map(|number| {
                let lhs = number.parse::<u64>()?;
                max_lhs = max_lhs.max(lhs);
                return Ok(lhs);
            }).collect::<Result<_>>()?;
            max_lhs_length = max_lhs_length.max(lhs.len());
            Ok((lhs, rhs))
        })
        .collect::<Result<_>>()?;

    let n_operators = 3 as u64;
    let hotpath_precomputed_1 = (0..max_lhs_length).map(|e| n_operators.pow(e as u32)).collect::<Vec<_>>();
    let hotpath_precomputed_2 = (0..max_lhs+1).map(|lhs| if lhs.eq(&0) { 0 } else { 10u64.pow(u64::ilog10(lhs) + 1) }).collect::<Vec<_>>();
    let mut total_calibration_result = 0;
    'equations: for (lhs, rhs_expected) in equations {
        let n_operator_places = lhs.len() - 1;
        let n_combinations = hotpath_precomputed_1[n_operator_places];
        let mut combination = 0u64;
        'combinations: while combination < n_combinations {
            let mut lhs_iter = lhs.iter();
            let mut rhs_actual = lhs_iter.next().with_context(|| format!("couldn't get 0th value of {}", stringify!(lhs_iter)))?.clone();
            for (lhs_ix, lhs) in lhs_iter.enumerate() {
                let denom = hotpath_precomputed_1[n_operator_places - lhs_ix - 1];
                let operation_digit = combination / denom % n_operators;
                match operation_digit {
                    0 => rhs_actual = hotpath_precomputed_2[lhs.clone() as usize] * rhs_actual + lhs, // Concat
                    1 => rhs_actual *= lhs, // Mul
                    2 => rhs_actual += lhs, // Add
                    _ => Err(anyhow!("bad {}: {}", stringify!(operation_digit), operation_digit))?,
                }
                if rhs_actual.gt(&rhs_expected) {
                    combination = (combination / denom + 1) * denom;
                    continue 'combinations;
                }
            }
            if rhs_actual.eq(&rhs_expected) {
                total_calibration_result += rhs_expected;
                continue 'equations;
            }
            combination += 1;
        }
    }

    fs::write("output", format!("{total_calibration_result}"))?;
    Ok(())
}