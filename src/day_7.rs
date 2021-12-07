use std::error::Error;

pub(crate) fn main(input: &str) -> Result<(), Box<dyn Error>> {
    let mut nums: Vec<u64> = input
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    #[cfg(feature = "part_1")]
    {
        nums.sort();

        let med = nums[nums.len() / 2];

        #[cfg(feature = "logging")]
        println!("med = {}", med);

        let cost: u64 = nums.iter().map(|&n| n.abs_diff(med)).sum();
        println!("{:?}", cost);
    }

    Ok(())
}
