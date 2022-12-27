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
struct Grid<T> {
    inner: [T; L * C],
}

// -------------------------
// -- Grid usage
// -------------------------

impl<T> Grid<T> where T: Copy{
    /// retrieve a value for itself coordinate in terms of lines/columns
    fn get(&self, line: usize, col: usize) -> T {
        self.inner[line * L + col]
    }

    fn lines(&self) -> impl Iterator<Item = &[T]> {
        self.inner.chunks(C)
    }

    fn line(&self, line_index: usize) -> &[T] {
        assert!(line_index < L, "trying to access line {line_index} while there are only {L} lines in the grid");
        self
            .lines()
            .enumerate()
            .find(|(i, _)| i ==& line_index)
            .map(|(_, values)| values).unwrap()
    }

    fn columns(&self) -> impl Iterator<Item = Vec<T>> + '_ {
        let iter_col_with_offset =
            |o: usize| self.inner.iter().skip(o).step_by(C).take(C).map(|c| *c);
        let all = (0..C)
            .into_iter()
            .map(move |col| iter_col_with_offset(col).collect::<Vec<T>>());

        all
    }

    fn column(&self, col_index: usize) -> Vec<T> {
        assert!(col_index < L, "trying to access line {col_index} while there are only {L} lines in the grid");
        self
            .columns()
            .enumerate()
            .find(|(i, _)| i ==&col_index)
            .map(|(_, values)| values).unwrap()
    }

}

// -------------------------
// -- Grid initialisation --
// -------------------------

/// Errors linked to the grid structure initialisation
#[derive(thiserror::Error, Debug)]
pub enum TryGridFromStrErr {
    #[error("Invalid char '{0}' encoured at position {1} of input while building Grid")]
    InvalidChar(char, usize),
}

/// Init Grid form a char iterator
impl<I> From<I> for Grid<u8>
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
                // println!("{index} -> {c}");
                instance.inner[index] = c.to_digit(10).unwrap() as u8;
                index += 1;
            } else {
                assert!(index % C == 0, "new line encountered after {index} chars while the column number is set as {C}");
            }
        }
        instance
    }
}

impl<T> Default for Grid<T> where T: Default + Copy {
    fn default() -> Self {
        Self { inner: [T::default(); L * C] }
    }
}

// -------------------------
// -- Forest trait
// -------------------------
fn p_u8s(a: &[u8]) {
    println!(
        "{}",
        a.iter().fold("".into(), |acc, u| format!("{acc} {u}"))
    );
}

trait Forest {
    fn data(&self) -> &[u8; C * L];
    fn height(&self, line :usize, col: usize) -> u8;
    fn is_visible(&self, line: usize, col: usize) -> bool;
    fn count_visible(&self) -> usize;
}

impl Forest for Grid<u8> {
    fn data(&self) -> &[u8; C * L] {
        &self.inner
    }

    fn height(&self, line: usize, col: usize) -> u8 {
        self.get(line, col)
    }

    fn is_visible(&self, line: usize, col: usize) -> bool {

        let h = self.height(line, col);
        let line_values = self.line(line);
        let columns_values = self.column(col);

        // vis from west : does some tree of heigth >= is found on the left
        let left_viz = line_values.iter().take(col).find(|v| **v > h).is_some();
        let right_viz = line_values.iter().rev().take(C - col).find(|v| **v > h).is_some();
        let top_viz = columns_values.iter().take(line).find(|v| **v > h).is_some();
        let bottom_viz = columns_values.iter().rev().take(L - line).find(|v| **v > h).is_some();

        left_viz || right_viz || top_viz || bottom_viz
    }

    fn count_visible(&self) -> usize {
        let mut count = 0;
        for l in 0..L {
            for c in 0..C {
                if self.is_visible(l, c) {
                    println!("({l}, {c}) is visible");
                    count += 1;
                } else {
                    println!("({l}, {c}) is NOT visible");
                }
            }
        }
        count
    }
}

// -------------------------
// -- MAIN
// -------------------------

fn main() -> Result<()> {
    println!("Count the number of visible trees in a forest !");
    color_eyre::install()?;
    let grid: Grid<u8> = EXAMPLE.chars().into();
    // dbg!(grid);
    Ok(())
}

// -------------------------
// -- TESTS
// -------------------------

#[cfg(test)]
mod test {
    use crate::{Forest, Grid, EXAMPLE};

    #[rustfmt::skip]
    const EXAMPLE_NB: [u8; 25] = [
        3,0,3,7,3,
        2,5,5,1,2,
        6,5,3,3,2,
        3,3,5,4,9,
        3,5,3,9,0];

    #[test]
    fn init_u8_grid_from_str() {
        let grid: Grid<u8> = EXAMPLE.chars().into();
        assert_eq!(grid.inner, EXAMPLE_NB);
    }

    #[test]
    fn get_value_at_position() {
        let grid: Grid<u8> = EXAMPLE.chars().into();
        assert_eq!(grid.get(0, 3), 7);
    }
    #[test]
    fn get_lines() {
        let grid: Grid<u8> = EXAMPLE.chars().into();
        for l in grid.lines() {
            println!(
                "{}",
                l.iter().fold("".into(), |acc, u| format!("{acc} {u}"))
            );
        }
        assert!(false);
    }
    #[test]
    fn get_columns() {
        let grid: Grid<u8> = EXAMPLE.chars().into();
        for l in grid.columns() {
            println!(
                "{}",
                l.iter().fold("".into(), |acc, u| format!("{acc} {u}"))
            );
        }
        assert!(false);
    }

    #[test]
    fn count_visible_trees() {
        let grid: Grid<u8> = EXAMPLE.chars().into();
        dbg!(grid.count_visible());
        assert!(false);
    }
}
