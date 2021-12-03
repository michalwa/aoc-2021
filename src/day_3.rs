use std::error::Error;

fn bit_sums<I, L>(lines: I) -> Vec<i32>
where
    I: IntoIterator<Item = L>,
    L: AsRef<str>,
{
    let mut sums = vec![];

    for line in lines {
        let len = line.as_ref().chars().count();
        sums.resize(sums.len().max(len), 0);

        for (sum, chr) in sums.iter_mut().zip(line.as_ref().chars()) {
            match chr {
                '1' => *sum += 1,
                '0' => *sum -= 1,
                _ => (),
            }
        }
    }

    sums
}

fn gamma_str<'a, I>(sums: I) -> String
where
    I: IntoIterator<Item = &'a i32>,
{
    sums
        .into_iter()
        .map(|&sum| if sum >= 0 { '1' } else { '0' })
        .collect()
}

fn epsilon_str<'a, I>(sums: I) -> String
where
    I: IntoIterator<Item = &'a i32>,
{
    sums
        .into_iter()
        .map(|&sum| if sum < 0 { '1' } else { '0' })
        .collect()
}

pub(crate) fn main(input: &str) -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "part_1")]
    {
        let sums = bit_sums(input.lines());
        let gamma = u32::from_str_radix(&gamma_str(&sums)[..], 2)?;
        let epsilon = u32::from_str_radix(&epsilon_str(&sums)[..], 2)?;

        println!(
            "gamma = {}; epsilon = {}; gamma * epsilon = {}",
            gamma, epsilon, gamma * epsilon,
        );
    }

    #[cfg(feature = "part_2")]
    {
        let length = input.lines().next().unwrap().len();
        let mut gamma_sieve = input.lines().collect::<Vec<_>>();
        let mut epsilon_sieve = gamma_sieve.clone();

        for i in 0..length {
            let gamma = gamma_str(&bit_sums(&gamma_sieve));
            let epsilon = epsilon_str(&bit_sums(&epsilon_sieve));

            if gamma_sieve.len() > 1 {
                gamma_sieve.retain(|n| n[i..(i + 1)] == gamma[i..(i + 1)]);
            }

            if epsilon_sieve.len() > 1 {
                epsilon_sieve.retain(|n| n[i..(i + 1)] == epsilon[i..(i + 1)]);
            }
        }

        let ogr = u32::from_str_radix(gamma_sieve.first().unwrap(), 2)?;
        let csr = u32::from_str_radix(epsilon_sieve.first().unwrap(), 2)?;

        println!("ogr = {}, csr = {}, ogr * csr = {}", ogr, csr, ogr * csr);
    }

    Ok(())
}
