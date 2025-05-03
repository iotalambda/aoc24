use std::fs;

use anyhow::{Context, Ok, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    let mut map = input
        .lines()
        .map(|row| row
            .chars()
            .map(|c| Loc::new(c))
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let mut xy_curr = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row
            .iter()
            .position(|l| l.c == '^')
            .map(|x| (x, y)))
        .next()
        .context("")?;

    {
        let (x_0, y_0) = xy_curr;
        let loc_0 = map.get_mut(y_0).and_then(|row| row.get_mut(x_0)).context("")?;
        loc_0.c = '.';
        loc_0.visited = true;
    }

    let mut dir_curr = Dir::Up;

    let mut count = 1;

    loop {
        let (x_curr, y_curr) = xy_curr;
        let (x_off, y_off): (isize, isize) = match dir_curr {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };

        let x_next = x_curr.wrapping_add_signed(x_off);
        let y_next = y_curr.wrapping_add_signed(y_off);

        let loc_next = map.get_mut(y_next).and_then(|row| row.get_mut(x_next));

        match loc_next {
            Some(x) if x.c == '.' => {
                if !x.visited {
                    count += 1;
                    x.visited = true;
                }
                xy_curr = (x_next, y_next);
            },
            Some(x) if x.c == '#' => {
                dir_curr = match dir_curr {
                    Dir::Up => Dir::Right,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                    Dir::Right => Dir::Down,
                };
            },
            None => break,
            _ => panic!(),
        };
    }

    fs::write("output", format!("{count}"))?;

    Ok(())
}

#[derive(Debug)]
struct Loc {
    c: char,
    visited: bool,
}

impl Loc {
    fn new(c: char) -> Self {
        Self {
            c,
            visited: false,
        }
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}