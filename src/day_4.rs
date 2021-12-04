use std::{
    error::Error,
    fmt::{self, Write},
};

#[derive(Clone)]
struct Cell {
    value: u32,
    checked: bool,
}

impl Cell {
    fn new(value: u32) -> Self {
        Self {
            value,
            checked: false,
        }
    }
}

struct Board<const N: usize> {
    cells: [[Cell; N]; N],
    won: bool,
}

impl<const N: usize> Board<N> {
    fn new<B, R>(cells: B) -> Self
    where
        B: AsRef<[R]>,
        R: AsRef<[Cell]>,
    {
        Self {
            cells: TryInto::<&[[Cell; N]; N]>::try_into(
                cells
                    .as_ref()
                    .iter()
                    .map(|row| {
                        TryInto::<&[Cell; N]>::try_into(row.as_ref())
                            .unwrap()
                            .clone()
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice()
                    .as_ref(),
            )
            .unwrap()
            .clone(),
            won: false,
        }
    }

    fn draw(&mut self, number: u32) -> Option<u32> {
        for cell in self.cells.iter_mut().flat_map(|row| row.iter_mut()) {
            if cell.value == number {
                cell.checked = true;
            }
        }

        if !self.won && self.is_winning() {
            self.won = true;
            Some(self.score() * number)
        } else {
            None
        }
    }

    fn is_winning(&self) -> bool {
        for row in 0..N {
            if self.cells[row].iter().all(|cell| cell.checked) {
                return true;
            }
        }

        for col in 0..N {
            if self.cells.iter().all(|row| row[col].checked) {
                return true;
            }
        }

        false
    }

    fn score(&self) -> u32 {
        self.cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| if !cell.checked { cell.value } else { 0 })
                    .sum::<u32>()
            })
            .sum()
    }
}

impl<const N: usize> fmt::Display for Board<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                if cell.checked {
                    f.write_fmt(format_args!("\x1b[90m{:>2}\x1b[0m ", cell.value))?;
                } else {
                    f.write_fmt(format_args!("{:>2} ", cell.value))?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

pub(crate) fn main(input: &str) -> Result<(), Box<dyn Error>> {
    const BOARD_SIZE: usize = 5;

    let mut lines = input.lines();

    let draws = lines.next().unwrap().split(',').map(str::parse);

    let mut boards: Vec<Vec<Vec<Cell>>> = vec![];

    for line in lines {
        if line.is_empty() {
            boards.push(vec![]);
        } else {
            boards.last_mut().unwrap().push(
                line.split_ascii_whitespace()
                    .map(|s| s.parse().map(Cell::new))
                    .collect::<Result<_, _>>()?,
            );
        }
    }

    let mut boards: Vec<Board<BOARD_SIZE>> = boards
        .into_iter()
        .map(Board::new)
        .collect();

    #[cfg(feature = "part_2")]
    let mut last_win = None;

    'outer: for draw in draws {
        let draw = draw?;
        println!("=== draw {} ===\n", draw);

        for board in &mut boards {
            if let Some(score) = board.draw(draw) {
                println!("{}", board);
                println!("won, score = {}\n", score);

                #[cfg(feature = "part_1")]
                break 'outer;

                #[cfg(feature = "part_2")]
                { last_win = Some(score); }
            } else {
                println!("{}", board);
            }
        }

        #[cfg(feature = "part_2")]
        {
            boards.retain(|board| !board.won);

            if boards.is_empty() {
                break 'outer;
            }
        }
    }

    #[cfg(feature = "part_2")]
    if let Some(last_win) = last_win {
        println!("score(last_to_win) = {}", last_win);
    }

    Ok(())
}
