use color_eyre::eyre::Result;

const EXAMPLE: &str = "
30373
25512
65332
33549
35390
";
const L: usize = 5;
const C: usize = 5;

/// A grid representing the trees
/// L is the fixed number of lines
/// C is the fixed number of columns
///
/// For the following grid L=4, C=3:
///     0  1  2
/// 0: [A][B][C]
/// 1: [D][E][F]
/// 2: [G][H][I]
/// 3: [J][K][L]
///
/// A = Grid[0][0]
/// B = Grid[0][1]
/// K = Grid[3][1]
///
#[derive(Debug)]
struct Grid {
    inner: [u8; L * C],
}

#[derive(thiserror::Error, Debug)]
pub enum TryGridFromStrErr {
    #[error("Invalid char '{0}' encoured at position {1} of input while building Grid")]
    InvalidChar(char, usize),
}

impl<I> From<I> for Grid
where
    I: Iterator<Item = char>,
{
    fn from(char_iter: I) -> Self {
        let mut instance = Grid::default();
        let mut index = 0;
        for c in char_iter {
            if c != '\n' {
                assert!(c >= '0' && c <= '9', "char is {}", c);
                assert!(
                    index < instance.inner.len(),
                    "{} is out of grid length ({})",
                    index,
                    instance.inner.len()
                );
                println!("{index} -> {c}");
                instance.inner[index] = c.to_digit(10).unwrap() as u8;
                index += 1;
            } else {
                assert!(index % C == 0, "new line encountered after {index} chars while the column number is set as {C}");
            }
        }
        instance
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self { inner: [0; L * C] }
    }
}

impl Grid {
    /// retrieve a value for itself coordinate in terms of lines/columns
    fn get(&self, line: usize, col: usize) -> u8 {
        self.inner[line * L + col]
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let grid: Grid = EXAMPLE.chars().into();
    dbg!(grid);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{Grid, EXAMPLE};

    #[rustfmt::skip]
    const EXAMPLE_NB: [u8; 25] = [
        3,0,3,7,3,
        2,5,5,1,2,
        6,5,3,3,2,
        3,3,5,4,9,
        3,5,3,9,0];

    #[test]
    fn init_u8_grid_from_str() {
        let grid: Grid = EXAMPLE.chars().into();
        assert_eq!(grid.inner, EXAMPLE_NB);
    }

    #[test]
    fn get_value_at_position() {
        let grid: Grid = EXAMPLE.chars().into();
        assert_eq!(grid.get(0, 3), 7);
    }
}
