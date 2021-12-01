#![feature(array_windows)]

use std::{
    fs::File,
    io::Read,
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let ns: Result<Vec<_>, _> = contents
        .lines()
        .map(|line| line.parse::<u32>())
        .collect();
    let ns = ns?; 

    // --- Part One ---
    // // ns.iter().zip(ns.iter().skip(1))
    //
    // let result = ns
    //     .array_windows::<2>()
    //     .filter(|[a, b]| a < b)
    //     .count();

    // --- Part Two ---
    let sums: Vec<_> = ns
        .array_windows::<3>()
        .map(|[a, b, c]| a + b + c)
        .collect();
    
    let result = sums
        .array_windows::<2>()
        .filter(|[a, b]| a < b)
        .count();

    println!("{:?}", result);

    Ok(())
}
