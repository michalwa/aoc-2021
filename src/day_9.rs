use std::{collections::HashSet, error::Error};

struct Grid<T> {
    cells: Vec<Vec<T>>,
    cols: usize,
    rows: usize,
}

impl<T> Grid<T> {
    fn parse<F>(input: &str, f: F) -> Self
    where
        F: Fn(char) -> T,
    {
        let cells: Vec<Vec<T>> = input
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().map(&f).collect())
            .collect();

        let cols = cells.get(0).map(Vec::len).unwrap_or(0);
        let rows = cells.len();

        Self { cells, cols, rows }
    }

    fn indices(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.cols).flat_map(move |x| (0..self.rows).map(move |y| (x, y, &self.cells[y][x])))
    }

    fn get(&self, col: isize, row: isize) -> Option<&T> {
        if col < 0 || col >= self.cols as isize || row < 0 || row >= self.rows as isize {
            None
        } else {
            Some(&self.cells[row as usize][col as usize])
        }
    }

    fn flood_count<B>(&self, x: usize, y: usize, is_boundary: B) -> usize
    where
        B: Fn(&T) -> bool,
    {
        let mut to_flood = HashSet::new();
        to_flood.insert((x, y));

        let mut flooded = HashSet::new();
        let mut next = HashSet::new();

        let mut count = 0;

        while !to_flood.is_empty() {
            for (x, y) in to_flood.drain() {
                if !is_boundary(&self.cells[y][x]) {
                    count += 1;

                    for (dx, dy) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
                        let (x, y) = (x as isize + dx, y as isize + dy);
                        if self.get(x, y).is_some() {
                            next.insert((x as usize, y as usize));
                        }
                    }

                    flooded.insert((x, y));
                }
            }

            to_flood.extend(next.drain().filter(|pos| !flooded.contains(pos)));
        }

        count
    }
}

pub(crate) fn main(input: &str) -> Result<(), Box<dyn Error>> {
    let heightmap = Grid::parse(input, |ch| ch.to_digit(10).unwrap() as u8);

    #[cfg(feature = "part_1")]
    let mut total_risk = 0u64;
    #[cfg(feature = "part_2")]
    let mut basins = vec![];

    for (x, y, &h) in heightmap.indices() {
        let mut risk = h + 1;

        for (dx, dy) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
            if let Some(&h2) = heightmap.get(x as isize + dx, y as isize + dy) {
                if h2 <= h {
                    risk = 0;
                    break;
                }
            }
        }

        #[cfg(feature = "part_1")]
        {
            total_risk += risk as u64;
        }

        #[cfg(feature = "part_2")]
        if risk > 0 {
            basins.push(heightmap.flood_count(x, y, |&h| h >= 9));
        }
    }

    #[cfg(feature = "part_1")]
    println!("{}", total_risk);

    #[cfg(feature = "part_2")]
    {
        basins.sort();
        let result = basins
            .into_iter()
            .rev()
            .take(3)
            .inspect(|b| print!("{} ", b))
            .fold(1, |a, b| a * b);

        println!("{}", result);
    }

    Ok(())
}
