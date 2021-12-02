use std::error::Error;

pub(crate) fn main(input: &str) -> Result<(), Box<dyn Error>> {
    let mut x = 0;
    let mut depth = 0;
    #[cfg(feature = "part_2")]
    let mut aim = 0;

    for line in input.lines() {
        let (dir, steps) = line.split_once(" ").unwrap();
        let steps = steps.parse::<i64>()?;

        #[cfg(feature = "part_1")]
        match dir {
            "forward" => x += steps,
            "up" => depth -= steps,
            "down" => depth += steps,
            _ => (),
        }

        #[cfg(feature = "part_2")]
        match dir {
            "forward" => { x += steps; depth += steps * aim; }
            "up" => aim -= steps,
            "down" => aim += steps,
            _ => (),
        }
    }

    println!("x = {}; depth = {}; x * depth = {}", x, depth, x * depth);

    Ok(())
}
