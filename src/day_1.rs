use std::error::Error;

pub(crate) fn main(input: &str) -> Result<(), Box<dyn Error>> {
    let ns: Vec<u32> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    #[cfg(feature = "part_1")]
    {
        let result = ns
            .array_windows() // ns.iter().zip(ns.iter().skip(1))
            .filter(|[a, b]| a < b)
            .count();

        println!("{:?}", result);
    }

    #[cfg(feature = "part_2")]
    {
        let result = ns
            .array_windows()
            .map(|[a, b, c]| a + b + c)
            .collect::<Vec<_>>()
            .array_windows()
            .filter(|[a, b]| a < b)
            .count();

        println!("{:?}", result);
    }

    Ok(())
}
