use std::fs;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    let len_input = input.len();

    let mut digits_dm_fw = input.lines().next().unwrap().chars().map(|c| c.to_digit(10).unwrap() as i64);
    let mut digits_dm_bw = digits_dm_fw.clone().rev();

    let mut ix_dm_fw: i64 = 0;
    let mut ix_dm_bw: i64 = len_input as i64;
    let mut ix_b_fw: i64 = 0;
    let mut id_b_bw: i64 = 0;
    let mut checksum: i64 = 0;
    let mut space_required: i64 = 0;
    let mut space_left: i64 = 0;
    loop {

        if space_required == 0 {
            while let Some(digit_dm_bw) = digits_dm_bw.next() {
                ix_dm_bw -= 1;
                if ix_dm_bw < ix_dm_fw {
                    break;
                }
                if ix_dm_bw % 2 == 0 {
                    id_b_bw = ix_dm_bw / 2;
                    space_required = digit_dm_bw;
                    break;
                }
            }
        }

        let space_allocated = if ix_dm_fw < ix_dm_bw {
            if space_left == 0 {
                let digit_dm_fw = digits_dm_fw.next().unwrap();
                let id_b_fw = ix_dm_fw / 2;
                checksum += id_b_fw * ((ix_b_fw * digit_dm_fw) + (digit_dm_fw * (digit_dm_fw - 1)) / 2);
                ix_b_fw += digit_dm_fw;
                space_left = digits_dm_fw.next().unwrap();
                ix_dm_fw += 2;
            }
            space_required.min(space_left)
        } else if space_required > 0 {
            space_required
        } else {
            break;
        };

        space_left -= space_allocated;
        space_required -= space_allocated;
        checksum += id_b_bw * ((ix_b_fw * space_allocated) + (space_allocated * (space_allocated - 1)) / 2);
        ix_b_fw += space_allocated;
    }

    fs::write("output", checksum.to_string())?;
    Ok(())
}