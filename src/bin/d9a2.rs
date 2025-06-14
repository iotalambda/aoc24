use std::fs;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    let len_input = input.len();

    let digits = input.lines().next().unwrap().chars().map(|c| c.to_digit(10).unwrap() as i64);

    let mut files = (0..=9).map(|_| Vec::with_capacity(len_input / 2 / 10)).collect::<Vec<_>>();

    #[derive(Clone, Copy)]
    struct File { id: i64, ix_b: i64 }
    let mut ix_b: i64 = 0;
    for (ix, d) in digits.clone().enumerate() {
        if ix % 2 == 0 {
            files[d as usize].push(File{ id: ix as i64 / 2, ix_b });
        }
        ix_b += d;
    }
    
    let mut checksum: i64 = 0;
    let mut ix_b: i64 = 0;
    for (ix, d) in digits.clone().enumerate() {
        let ix = ix as i64;
        if ix % 2 == 1 {
            let mut space_left = d;
            loop {
                let mut best_file_and_size: Option<(File, usize)> = None;
                for size in 0..=space_left {
                    let size = size as usize;
                    best_file_and_size = match files[size].last() {
                        Some(&file) if ix_b < file.ix_b && best_file_and_size.is_none_or(|(best_file, _)| best_file.id < file.id) => Some((file, size)),
                        _ => best_file_and_size,
                    }
                }
                match best_file_and_size {
                    Some((file, size)) => {
                        files[size].pop();
                        let size = size as i64;
                        checksum += file.id * (size * (ix_b + d - space_left) + (size * (size - 1)) / 2);
                        space_left -= size;
                        if space_left == 0 {
                            break;
                        }
                    },
                    _ => break,
                }
            }
        }
        ix_b += d;
    }

    for (size, files) in files.iter().enumerate() {
        let size = size as i64;
        for file in files {
            checksum += file.id * (size * file.ix_b + (size * (size - 1)) / 2);
        }
    }

    fs::write("output", checksum.to_string())?;
    Ok(())
}