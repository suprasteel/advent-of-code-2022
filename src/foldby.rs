/// This instance reduce an existing iterator, grouping concecutive elements by the provided value.
/// The reducer is called `by` times on the successive elements and behaves like a fold.
/// The previous computation is given back to the reducer.
/// The reducer takes an accumulator and the current value: (acc: T, cur: I::Item) -> {}
pub struct FoldBy<I, F>
where
    I: Iterator,
    I::Item: Default,
    F: Fn(&I::Item, I::Item) -> I::Item,
{
    iter: I,
    init: I::Item,
    by: usize,
    reducer: F,
}

impl<I, F> FoldBy<I, F>
where
    I: Iterator,
    I::Item: Default,
    F: Fn(&I::Item, I::Item) -> I::Item,
{
    fn new(iter: I, by: usize, init: I::Item, reducer: F) -> Self {
        Self {
            iter,
            init,
            by,
            reducer,
        }
    }
}

impl<I, F> Iterator for FoldBy<I, F>
where
    I: Iterator,
    I::Item: Default + std::fmt::Debug,
    F: Fn(&I::Item, I::Item) -> I::Item,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut acc = None;
        for _ in 0..self.by {
            let next = self.iter.next();
            if next.is_none() {
                return acc;
            }
            acc = next.map(|v| (self.reducer)(acc.as_ref().unwrap_or(&self.init), v));
        }
        acc
    }
}

pub trait FoldByExt: Iterator {
    fn fold_by<F>(self, by: usize, init: Self::Item, reducer: F) -> FoldBy<Self, F>
    where
        Self: Sized,
        Self::Item: Default + Clone,
        F: Fn(&Self::Item, Self::Item) -> Self::Item,
    {
        assert_ne!(by, 0);
        FoldBy::new(self, by, init, reducer)
    }
}

/// Implement my reduce for all t being an iterator
impl<T> FoldByExt for T
where
    T: Iterator,
    T::Item: Default + Clone,
{
    fn fold_by<F>(self, by: usize, init: Self::Item, reducer: F) -> FoldBy<Self, F>
    where
        Self: Sized,
        Self::Item: Default + Clone,
        F: Fn(&Self::Item, Self::Item) -> Self::Item,
    {
        assert_ne!(by, 0);
        FoldBy::new(self, by, init, reducer)
    }
}

#[cfg(test)]
mod test {

    use super::FoldByExt;

    #[test]
    fn it_iterates_over_elements_by_one() {
        let elmts: Vec<u8> = (0..3_u8).fold_by(1, 10, |acc, cur| acc * cur).collect();
        assert_eq!(elmts, vec![0, 10, 20]);
    }

    #[test]
    fn it_iterates_over_elements_by_two() {
        let elmts: Vec<u8> = (0..6_u8).fold_by(2, 0, |acc, cur| acc + cur).collect();
        assert_eq!(elmts, vec![1, 5, 9]);
    }

    #[test]
    fn it_uses_incomplete_sequences() {
        let elmts: Vec<u8> = (0..1_u8).fold_by(4, 0, |acc, cur| acc + cur).collect();
        assert_eq!(elmts, vec![0]);
    }

    #[test]
    fn it_works_with_chars() {
        let elmts: Vec<char> = "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .fold_by(3, 'a', |acc, cur| {
                ((*acc as u8 + cur as u8) % 26 + 65) as char
            })
            .collect();
        dbg!(&elmts);
        assert_eq!(elmts, vec!['B', 'K', 'T', 'C', 'L', 'U', 'D', 'M', 'P']);
    }

    /*
    // Modifier le parametre init pour en faire un type O indépendant
    #[test]
    fn it_works_with_differents_in_out_types() {
        let elmts: Vec<u8> = "abcdefghijklmnopqrstuvwxyz".chars().fold_by(4, String::with_capacity(3), |acc, cur| acc.concat(cur)).collect();
        assert_eq!(elmts, vec![0]);
    }
    */
}
