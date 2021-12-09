use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Write},
    hash::{Hash, Hasher},
    ops,
    str::FromStr,
};

const DIGITS: [Pattern; 10] = [
    //                        GFEDCBA
    Pattern::from_bit_set(0b0_1110111),
    Pattern::from_bit_set(0b0_0100100),
    Pattern::from_bit_set(0b0_1011101),
    Pattern::from_bit_set(0b0_1101101),
    Pattern::from_bit_set(0b0_0101110),
    Pattern::from_bit_set(0b0_1101011),
    Pattern::from_bit_set(0b0_1111011),
    Pattern::from_bit_set(0b0_0100101),
    Pattern::from_bit_set(0b0_1111111),
    Pattern::from_bit_set(0b0_1101111),
];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pattern {
    set: u8,
    unbound: u8,
}

impl FromStr for Pattern {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            set: s
                .chars()
                .map(|ch| match ch {
                    'a'..='g' => 1 << (ch as u32 - 'a' as u32),
                    _ => 0,
                })
                .fold(0, |a, b| a | b),
            unbound: 0,
        })
    }
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.len() == 0 {
            return f.write_str("{}");
        }

        let chars = if f.alternate() { 'A'..='G' } else { 'a'..='g' };

        for (i, ch) in chars.enumerate() {
            if self.set & (1 << i) != 0 {
                f.write_char(ch)?;
            }
        }

        for _ in 0..self.unbound {
            f.write_char('?')?;
        }

        Ok(())
    }
}

impl ops::Sub<Pattern> for Pattern {
    type Output = Pattern;

    fn sub(self, rhs: Pattern) -> Self::Output {
        Self {
            set: self.set & !rhs.set,
            unbound: self.unbound,
        }
    }
}

impl ops::BitAnd<Pattern> for Pattern {
    type Output = Pattern;

    fn bitand(self, rhs: Pattern) -> Self::Output {
        Self {
            set: self.set & rhs.set,
            unbound: 0,
        }
    }
}

impl ops::Mul<Pattern> for Pattern {
    type Output = Pattern;

    fn mul(self, rhs: Pattern) -> Self::Output {
        assert!(self.len() == rhs.len());
        let mut merged = self & rhs;
        merged.unbound = (self.len() - merged.len()) as u8;
        merged
    }
}

impl Pattern {
    const fn from_bit_set(set: u8) -> Self {
        Self { set, unbound: 0 }
    }

    const fn len(self) -> usize {
        self.set.count_ones() as usize + self.unbound as usize
    }

    const fn len_bound(self) -> usize {
        self.set.count_ones() as usize
    }

    const fn singleton_index(self) -> Option<u8> {
        if self.unbound == 0 && self.set != 0 && self.set.is_power_of_two() {
            Some(self.set.log2() as u8)
        } else {
            None
        }
    }

    const fn contains(self, rhs: Self) -> bool {
        self.set & rhs.set == rhs.set && rhs.unbound == 0
    }
}

#[derive(Default)]
struct Solver {
    knowledge: HashSet<(Pattern, Pattern)>,
}

impl Solver {
    fn learn(&mut self, example: Pattern) {
        if let Some(pattern) = DIGITS
            .iter()
            .copied()
            .filter(|d| d.len() == example.len())
            .reduce(|a, b| a * b)
        {
            Self::remember(&mut self.knowledge, pattern, example);
        }
    }

    fn remember(k: &mut HashSet<(Pattern, Pattern)>, a: Pattern, b: Pattern) {
        if a.len_bound() > 0 && a.len() == b.len() {
            k.insert((a, b));
        }
    }

    fn solve(&mut self, iterations: usize) -> Mapping {
        for _ in 0..iterations {
            let mut new_knowledge = HashSet::new();

            for (&(a, b), &(c, d)) in self
                .knowledge
                .iter()
                .flat_map(|a| self.knowledge.iter().map(move |b| (a, b)))
            {
                if a == c {
                    continue;
                }

                if a.contains(c) && b.contains(d) {
                    Self::remember(&mut new_knowledge, a - c, b - d);
                }

                let (i, j) = (a & c, b & d);
                if i.len() == j.len() {
                    Self::remember(&mut new_knowledge, i, j);
                }
            }

            self.knowledge.extend(new_knowledge.drain());
        }

        let mut mapping = Mapping([0; 7]);

        for &(k, v) in &self.knowledge {
            if let Some(i) = k.singleton_index() {
                mapping.0[i as usize] = v.set;
            }
        }

        mapping
    }
}

#[derive(Clone, Copy)]
struct Mapping([u8; 7]);

impl fmt::Debug for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut original = 1;
        for &mapped in &self.0 {
            let a = Pattern::from_bit_set(original);
            let b = Pattern::from_bit_set(mapped);
            f.write_fmt(format_args!("{:#?} = {:?}\n", a, b))?;
            original <<= 1;
        }
        Ok(())
    }
}

impl Mapping {
    fn is_valid(&self) -> bool {
        self.0.iter().all(|&m| m != 0 && m.is_power_of_two())
    }

    fn decode(&self, input: Pattern) -> Pattern {
        let mut result = 0;

        let mut original = 1;
        for &mapped in &self.0 {
            debug_assert!(mapped != 0 && mapped.is_power_of_two());

            if input.set & mapped != 0 {
                result |= original;
            }
            original <<= 1;
        }

        Pattern {
            set: result,
            unbound: 0,
        }
    }
}

pub(crate) fn main(input: &str) -> Result<(), Box<dyn Error>> {
    let mut sum = 0;

    for line in input.lines() {
        let (examples, inputs) = line.split_once(" | ").unwrap();

        let mut solver = Solver::default();
        for example in examples
            .split_ascii_whitespace()
            .map(|e| e.parse().unwrap())
        {
            solver.learn(example);
        }

        let mapping = solver.solve(3);
        assert!(mapping.is_valid());

        #[cfg(feature = "logging")]
        println!("{:?}", mapping);

        let inputs = inputs.split_ascii_whitespace();

        let mut result = 0;
        for digit in inputs
            .map(|i| i.parse().unwrap())
            .map(|i| mapping.decode(i))
            .map(|d| DIGITS.iter().position(|&p| p == d).unwrap())
        {
            #[cfg(feature = "part_1")]
            if [1, 4, 7, 8].contains(&digit) {
                result += 1;
            }

            #[cfg(feature = "part_2")]
            { result = result * 10 + digit; }
        }

        #[cfg(feature = "logging")]
        println!("result = {}\n", result);

        sum += result;
    }

    println!("sum = {}", sum);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mapping_decode() {
        let identity = Mapping([
            0b0_0000001,
            0b0_0000010,
            0b0_0000100,
            0b0_0001000,
            0b0_0010000,
            0b0_0100000,
            0b0_1000000,
        ]);

        assert!(identity.is_valid());

        for i in (0..=0b0_1111111).map(Pattern::from_bit_set) {
            assert_eq!(i, identity.decode(i));
        }
    }
}
