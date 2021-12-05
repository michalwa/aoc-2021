use std::{error::Error, ops::Add, str::FromStr};

pub(crate) fn main(input: &str) -> Result<(), Box<dyn Error>> {
    const SIZE: usize = 1000;
    let mut grid = vec![0; SIZE * SIZE].into_boxed_slice();

    for (i, line) in input.lines().enumerate() {
        let (a, b) = line.split_once(" -> ").ok_or("malformed input")?;
        let a = a.split_once(',').ok_or("malformed input")?;
        let b = b.split_once(',').ok_or("malformed input")?;

        let a: (usize, usize) = (a.0.parse()?, a.1.parse()?);
        let b: (usize, usize) = (b.0.parse()?, b.1.parse()?);

        if a.0 > SIZE || a.1 > SIZE || b.0 > SIZE || b.1 > SIZE {
            panic!("Max size ({}) exceeded on line {}", SIZE, i);
        }

        #[cfg(feature = "part_1")]
        if a.0 != b.0 && a.1 != b.1 { continue }

        let step = (
            (b.0 as isize - a.0 as isize).signum(),
            (b.1 as isize - a.1 as isize).signum(),
        );

        println!("Line from {:?} to {:?} step {:?}", a, b, step);

        let mut pos = a;
        loop {
            grid[pos.1 * SIZE + pos.0] += 1;

            if pos == b { break }

            pos = (
                pos.0.saturating_add_signed(step.0),
                pos.1.saturating_add_signed(step.1),
            );
        }
    }

    let answer = grid.iter().filter(|&&cell| cell >= 2).count();
    println!("{}", answer);

    Ok(())
}
