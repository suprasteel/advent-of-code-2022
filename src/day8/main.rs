use std::{
    fmt::{self, Display},
    fs::read_to_string,
};

use color_eyre::eyre::Result;

const EXAMPLE: &str = "
30373
25512
65332
33549
35390
";

#[cfg(test)]
mod Constants {
    pub const L: usize = 5;
    pub const C: usize = 5;
    pub const Z: usize = L * C;
}

#[cfg(not(test))]
mod Constants {
    pub const L: usize = 99;
    pub const C: usize = 99;
    pub const Z: usize = L * C;
}

const L: usize = Constants::L;
const C: usize = Constants::C;
const Z: usize = Constants::Z;

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
struct Grid<T, const Z: usize> {
    // L * C
    inner: [T; Z],
}

// -------------------------
// -- Grid usage
// -------------------------
#[inline]
fn assert_coords(line: usize, col: usize) {
    assert_line(line);
    assert_col(col);
}

#[inline]
fn assert_line(line: usize) {
    assert!(line < L, "Line {line} > grid size ({L})");
}
#[inline]
fn assert_col(col: usize) {
    assert!(col < C, "Column {col} > grid size ({C})");
}

impl<T, const Z: usize> Grid<T, Z>
where
    T: Copy,
{
    /// retrieve a value for itself coordinate in terms of lines/columns
    fn get(&self, line: usize, col: usize) -> T {
        assert_coords(line, col);
        self.inner[line * L + col]
    }

    fn get_mut(&mut self, line: usize, col: usize) -> &mut T {
        assert_coords(line, col);
        &mut self.inner[line * L + col]
    }

    fn lines(&self) -> impl Iterator<Item = &[T]> {
        self.inner.chunks(C)
    }

    fn line(&self, line_index: usize) -> &[T] {
        assert_line(line_index);

        self.lines()
            .enumerate()
            .find(|(i, _)| i == &line_index)
            .map(|(_, values)| values)
            .unwrap()
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
        assert_col(col_index);
        self.columns()
            .enumerate()
            .find(|(i, _)| i == &col_index)
            .map(|(_, values)| values)
            .unwrap()
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
impl<I, const Z: usize> From<I> for Grid<u8, Z>
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
                    "{index} is out of grid length (L({L})*C({C})={})",
                    instance.inner.len()
                );
                instance.inner[index] = c.to_digit(10).unwrap() as u8;
                index += 1;
            } else {
                assert!(index % C == 0, "new line encountered after {index} chars while the column number is set as {C}");
            }
        }
        instance
    }
}

impl<T, const Z: usize> Default for Grid<T, Z>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            inner: [T::default(); Z],
        }
    }
}

// -------------------------
// -- Forest trait
// -------------------------

/// get the scenic score of the drection,
/// pass the tree height and the slice of item in one direction, including the tree on which we
/// compute the score
fn slice_scenic_score(view: &[u8], height: u8) -> usize {
    view.iter()
        .position(|tree_h| tree_h >= &height)
        .map(|i| i + 1)
        .unwrap_or_else(|| view.len())
}

trait Forest<const Z: usize> {
    fn data(&self) -> &[u8; Z];
    fn height(&self, line: usize, col: usize) -> u8;
    fn is_visible(&self, line: usize, col: usize) -> bool;
    fn count_visible(&self) -> usize;
    fn tree_scenic_score(&self, line: usize, col: usize) -> usize;
    fn scenic_score(&self) -> usize;
    fn max_scenic_score(&self) -> (usize, usize, usize); // (l, c, val)
}

impl<const Z: usize> Forest<Z> for Grid<u8, Z> {
    fn data(&self) -> &[u8; Z] {
        &self.inner
    }

    fn height(&self, line: usize, col: usize) -> u8 {
        self.get(line, col)
    }

    fn is_visible(&self, line: usize, col: usize) -> bool {
        let h = self.height(line, col);
        let line_values = self.line(line);
        let columns_values = self.column(col);

        fn visible_from_start(line: &[u8], pos: usize, h: u8) -> bool {
            pos == 0 || line.iter().take(pos).find(|v| **v >= h).is_none()
        }
        fn visible_from_end(line: &[u8], pos: usize, h: u8) -> bool {
            pos == line.len() - 1
                || line
                    .iter()
                    .rev()
                    .take(line.len() - (pos + 1))
                    .find(|v| **v >= h)
                    .is_none()
        }

        let left_viz = visible_from_start(line_values, col, h);
        let right_viz = visible_from_end(line_values, col, h);
        let top_viz = visible_from_start(columns_values.as_ref(), line, h);
        let bottom_viz = visible_from_end(columns_values.as_ref(), line, h);

        log::debug!("Computing visibility of ({line}, {col}): (left, right, top, bottom) = ({left_viz}, {right_viz}, {top_viz}, {bottom_viz})");

        left_viz || right_viz || top_viz || bottom_viz
    }

    fn count_visible(&self) -> usize {
        let mut count = 0;
        let mut viz_grid: Grid<bool, Z> = Grid::default();
        for l in 0..L {
            for c in 0..C {
                if self.is_visible(l, c) {
                    *viz_grid.get_mut(l, c) = true;
                    count += 1;
                } else {
                    *viz_grid.get_mut(l, c) = false;
                }
            }
        }
        log::debug!("{viz_grid}");
        count
    }

    fn tree_scenic_score(&self, line: usize, col: usize) -> usize {
        let h = self.height(line, col);
        let line_values = self.line(line);
        let columns_values = self.column(col);

        let (left, right) = line_values.split_at(col);
        let (top, bottom) = columns_values.split_at(line);
        let left_score = slice_scenic_score(left, h);
        let right_score = slice_scenic_score(&right[1..], h);
        let top_score = slice_scenic_score(top, h);
        let bottom_score = slice_scenic_score(&bottom[1..], h);
        left_score * right_score * top_score * bottom_score
    }

    fn scenic_score(&self) -> usize {
        let mut score = 0;
        for l in 0..L {
            for c in 0..C {
                score += self.tree_scenic_score(l, c);
            }
        }
        score
    }

    fn max_scenic_score(&self) -> (usize, usize, usize) {
        let mut max_score = 0;
        let mut coords = (0, 0);
        for l in 0..L {
            for c in 0..C {
                let score = self.tree_scenic_score(l, c);
                println!("{l}:{c}  >  {score}");
                if score > max_score {
                    max_score = score;
                    coords = (l, c);
                } 
            }
        }
        (coords.0, coords.1, max_score)
    }
}

impl<T, const Z: usize> Display for Grid<T, Z>
where
    T: Display + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.lines().fold(String::from(""), |all, l| {
            format!(
                "{}\n{}",
                all,
                l.iter().fold("".into(), |acc, v| format!("{} {}", acc, v))
            )
        });
        write!(f, "{}", s)
    }
}

// -------------------------
// -- MAIN
// -------------------------

fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    log::info!("Count the number of visible trees in a forest !");
    color_eyre::install()?;
    let data = read_to_string("./data/day8.dat")?;
    let grid: Grid<u8, { C * L }> = data.chars().into();
    let visibles = grid.count_visible();
    let (scenic_l, scenic_c, max_scenic_score) = grid.max_scenic_score();
    log::info!("The number of visible trees in this forest is {visibles}");
    log::info!("The top scenic score is {max_scenic_score} for tree ({scenic_l}, {scenic_c})");
    Ok(())
}

// -------------------------
// -- TESTS
// -------------------------

#[cfg(test)]
mod test {
    use crate::{slice_scenic_score, Forest, Grid, C, EXAMPLE, L, Z};

    #[rustfmt::skip]
    const EXAMPLE_NB: [u8; Z] = [
        3,0,3,7,3,
        2,5,5,1,2,
        6,5,3,3,2,
        3,3,5,4,9,
        3,5,3,9,0];

    #[test]
    fn init_u8_grid_from_str() {
        let grid: Grid<u8, Z> = EXAMPLE.chars().into();
        assert_eq!(grid.inner, EXAMPLE_NB);
    }

    #[test]
    fn get_value_at_position() {
        let grid: Grid<u8, Z> = EXAMPLE.chars().into();
        assert_eq!(grid.get(0, 3), 7);
    }
    #[test]
    fn get_lines() {
        let grid: Grid<u8, Z> = EXAMPLE.chars().into();
        for (i, l) in grid.lines().enumerate() {
            log::debug!(
                "{}",
                l.iter().fold("".into(), |acc, u| format!("{acc} {u}"))
            );
            assert_eq!(l, &EXAMPLE_NB[i * C..(i * L + C)]);
        }
    }
    #[test]
    fn get_columns() {
        let grid: Grid<u8, Z> = EXAMPLE.chars().into();
        assert_eq!(grid.columns().next().unwrap(), vec![3, 2, 6, 3, 3]);
    }

    #[test]
    fn count_visible_trees() {
        let grid: Grid<u8, Z> = EXAMPLE.chars().into();
        assert_eq!(grid.count_visible(), 21);
    }

    #[test]
    fn scenic_score_from_slice() {
        let slices = [
            (&[6, 4, 5, 2, 1, 6, 7], 5),
            (&[9, 1, 0, 0, 5, 5, 5], 6),
            (&[0, 1, 0, 0, 5, 5, 5], 1),
            (&[9, 1, 3, 5, 7, 9, 0], 5),
        ];
        for (input, expected) in slices {
            let (h, view) = input.split_at(1);
            assert_eq!(slice_scenic_score(view, h[0]), expected);
        }
            assert_eq!(slice_scenic_score(&[], 5), 0);

    }

    #[test]
    fn example_max_scenic_score() {
        let grid: Grid<u8, Z> = EXAMPLE.chars().into();
        assert_eq!(grid.max_scenic_score(), (3, 2, 8));
        
    }
}
