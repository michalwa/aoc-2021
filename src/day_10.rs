use std::error::Error;

fn closing(opening: char) -> Option<char> {
    Some(match opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => return None,
    })
}

fn validate(line: &str) -> (Option<char>, Option<String>) {
    let mut stack = vec![];

    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => {
                if let Some(&last) = stack.last() {
                    if closing(last) == Some(ch) {
                        stack.pop();
                    } else {
                        return (Some(ch), None);
                    }
                } else {
                    return (Some(ch), None);
                }
            }
            _ => (),
        }
    }

    (
        None,
        if stack.is_empty() {
            None
        } else {
            Some(stack.into_iter().rev().filter_map(closing).collect())
        },
    )
}

pub(crate) fn main(input: &str) -> Result<(), Box<dyn Error>> {
    let lines = input.lines().map(str::trim).filter(|line| !line.is_empty());

    #[cfg(feature = "part_1")]
    {
        let result: u64 = lines
            .clone()
            .filter_map(|line| validate(line).0)
            .map(|ch| match ch {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            })
            .sum();

        println!("{}", result);
    }

    #[cfg(feature = "part_2")]
    {
        let mut results: Vec<_> = lines
            .clone()
            .filter_map(|line| validate(line).1)
            .map(|comp| {
                comp.chars()
                    .map(|ch| match ch {
                        ')' => 1u64,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => 0,
                    })
                    .fold(0, |a, b| a * 5 + b)
            })
            .collect();

        results.sort();

        let med = results[results.len() / 2];

        println!("{}", med);
    }

    Ok(())
}
