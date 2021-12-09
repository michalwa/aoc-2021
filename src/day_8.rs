use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Write},
    hash::Hash,
    ops,
    str::FromStr,
};

/// Patterns representing digits from 0 to 9
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

/// A bit set which can represent the state of a 7-segment display
/// with an additional number of "unbound" elements. No assumptions are
/// made about the equality of unbound elements.
///
/// Examples:
///  - a pattern representing the digit 2: `ACDEG`
///  - a pattern representing the "intersection" of 5-segment digits: `ADG??`
///
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

    /// Subtracts bound elements of the right-hand-side set from this set.
    /// Leaves unbound elements unchanged.
    fn sub(self, rhs: Pattern) -> Self::Output {
        Self {
            set: self.set & !rhs.set,
            unbound: self.unbound,
        }
    }
}

impl ops::BitAnd<Pattern> for Pattern {
    type Output = Pattern;

    /// Computes the intersection of the two sets.
    /// The resulting set is guaranteed to contain no unbound elements.
    fn bitand(self, rhs: Pattern) -> Self::Output {
        Self {
            set: self.set & rhs.set,
            unbound: 0,
        }
    }
}

impl ops::Mul<Pattern> for Pattern {
    type Output = Pattern;

    /// "Merges" the two sets together so that the resulting set contains all common elements
    /// as bound elements and a number of unbound elements, such that the length of the
    /// resulting set is equal to the length of both sets.
    ///
    /// # Safety
    /// Requires that the lengths of both sets are equal, otherwise panics.
    fn mul(self, rhs: Pattern) -> Self::Output {
        assert!(self.len() == rhs.len());
        let mut merged = self & rhs;
        merged.unbound = (self.len() - merged.len()) as u8;
        merged
    }
}

impl Pattern {
    /// Constructs a pattern with bound elements given by the bit set and no unbound elements.
    const fn from_bit_set(set: u8) -> Self {
        Self { set, unbound: 0 }
    }

    /// Returns the total number of elements (both bound and unbound) in the set.
    const fn len(self) -> usize {
        self.set.count_ones() as usize + self.unbound as usize
    }

    /// Returns the number of bound elements in the set.
    const fn len_bound(self) -> usize {
        self.set.count_ones() as usize
    }

    /// Returns the bit index (0 is least significant) of the single bound element in this set,
    /// only if the set contains exactly one element which is bound. Returns `None` otherwise.
    const fn singleton_index(self) -> Option<u8> {
        if self.unbound == 0 && self.set != 0 && self.set.is_power_of_two() {
            Some(self.set.log2() as u8)
        } else {
            None
        }
    }

    /// Returns `true` if all bound elements of the right-hand-side set are also contained in
    /// this set.
    ///
    /// Always returns `false` if the right-hand-side set contains unbound elements, since they
    /// cannot be compared for equality.
    const fn contains(self, rhs: Self) -> bool {
        self.set & rhs.set == rhs.set && rhs.unbound == 0
    }
}

/// Resolves display segment mappings based on example patterns
#[derive(Default)]
struct Solver {
    /// Each entry `(a, b)` represents the assertion `a = b` (equality of sets, unrelated to [`Eq`]),
    /// e.g. `(adg??, bcdef)` means `{A,D,G,?,?} = {b,c,d,e,f}`, where the left-hand-side
    /// represents the "original" pattern and the right-hand-side represents the mapped pattern.
    knowledge: HashSet<(Pattern, Pattern)>,
}

impl Solver {
    /// Processes the given example pattern storing collected information for later use.
    /// The pattern (of length `n`) represents the result of mapping a `n`-segment digit
    /// using the solved mapping. Behavior is unpredictable if two contradictory examples
    /// are given to this method.
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

    /// Stores the given mapping in the knowledge set, only if it is valid and useful,
    /// i.e. `a` contains one or more bound elements and both patterns are of equal length.
    fn remember(k: &mut HashSet<(Pattern, Pattern)>, a: Pattern, b: Pattern) {
        if a.len_bound() > 0 && a.len() == b.len() {
            k.insert((a, b));
        }
    }

    /// Iterates the solving algorithm the given number of times and returns the resulting mapping.
    /// The mapping is not guaranteed to be valid (`.is_valid()`), e.g. if not enough example
    /// patterns were given or not enough iterations have been executed.
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

                // For each pair of equalities compute their differences and intersections
                //
                //   (A = B) ⋀ (C = D) => (A ∖ C = B ∖ D) ⋀ (A ⋂ C = B ⋂ D)
                //
                if a.contains(c) && b.contains(d) {
                    Self::remember(&mut new_knowledge, a - c, b - d);
                }
                Self::remember(&mut new_knowledge, a & c, b & d);
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

/// Stores the solved segment mapping
/// (`mapping[original_segment]` = singleton bit set representing the mapped segment)
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

    /// Reverses the solved mapping on the given pattern, producing the original pattern
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

        // Train the solver on the examples
        let mut solver = Solver::default();
        for example in examples
            .split_ascii_whitespace()
            .map(|e| e.parse().unwrap())
        {
            solver.learn(example);
        }

        // Solve the mapping
        let mapping = solver.solve(3);
        assert!(mapping.is_valid());

        #[cfg(feature = "logging")]
        println!("{:?}", mapping);

        // Reverse the mapping on the inputs
        let inputs = inputs.split_ascii_whitespace();

        let mut result = 0;
        for digit in inputs
            .map(|i| i.parse().unwrap())
            .map(|i| mapping.decode(i))
            // Find digit indices matching the digit patterns
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
