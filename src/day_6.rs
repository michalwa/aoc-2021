use std::error::Error;

pub(crate) fn main(input: &str) -> Result<(), Box<dyn Error>> {
    const DAYS: usize = 256;
    const RESET_TIMEOUT: usize = 6;
    const NEWBORN_TIMEOUT: usize = 8;

    debug_assert!(RESET_TIMEOUT <= NEWBORN_TIMEOUT);

    let mut fish: [usize; NEWBORN_TIMEOUT + 1] = [0; NEWBORN_TIMEOUT + 1];

    for initial_fish in input
        .split(',')
        .map(str::trim)
    {
        let initial_fish: usize = initial_fish.parse()?;
        fish[initial_fish] += 1;
    }

    #[cfg(feature = "logging")]
    println!("Initial state: {:?}", fish);

    for day in 0..DAYS {
        let newborn = fish[0];

        for timeout in 1..=NEWBORN_TIMEOUT {
            fish[timeout - 1] = fish[timeout];
        }

        fish[RESET_TIMEOUT] += newborn;
        fish[NEWBORN_TIMEOUT] = newborn;

        #[cfg(feature = "logging")]
        println!("After {:>2} days: {:?}", day + 1, fish);
    }

    println!("{}", fish.iter().sum::<usize>());

    Ok(())
}
