use color_eyre::eyre::Result;

const EXAMPLE: &str = "
30373
25512
65332
33549
35390
";
const L: usize = 5;
const C: usize = 6;

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
                assert!(index < instance.inner.len(), "{} is out of grid length ({})", index, instance.inner.len());
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

#[derive(thiserror::Error, Debug)]
pub enum TryGridFromStrErr {
    #[error("Invalid char '{0}' encoured at position {1} of input while building Grid")]
    InvalidChar(char, usize),
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let grid: Grid = EXAMPLE.chars().into();
    dbg!(grid);
    Ok(())
}
