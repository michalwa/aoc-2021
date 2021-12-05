#![feature(array_windows)]
#![feature(mixed_integer_ops)]

use std::{error::Error, env, fs::File, io::Read};

#[cfg(feature = "day_1")]
mod day_1;
#[cfg(feature = "day_2")]
mod day_2;
#[cfg(feature = "day_3")]
mod day_3;
#[cfg(feature = "day_4")]
mod day_4;
#[cfg(feature = "day_5")]
mod day_5;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let filename = args.next().unwrap();

    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    #[cfg(feature = "day_1")]
    day_1::main(&input[..])?;
    #[cfg(feature = "day_2")]
    day_2::main(&input[..])?;
    #[cfg(feature = "day_3")]
    day_3::main(&input[..])?;
    #[cfg(feature = "day_4")]
    day_4::main(&input[..])?;
    #[cfg(feature = "day_5")]
    day_5::main(&input[..])?;

    Ok(())
}
