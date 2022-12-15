use advent_of_code::foldby::FoldByExt;
use std::fs::read_to_string;

const VARIANT: u8 = 2;

const fn get_code_variant() -> fn(&str) {
    match VARIANT {
        1 => find_uneven_item,
        2 => determine_badge_for_triplets,
        _ => panic!("not a valid variant"),
    }
}

fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    get_code_variant()(read_to_string("./data/day3.dat")?.as_str());
    Ok(())
}

/// Day 3 part 2
fn determine_badge_for_triplets(bags_string: &str) {
    let result: u32 = bags_string
        .lines()
        .map(compute_bitflags_for_string)
        .fold_by(3, u64::MAX, |acc, cur| acc & cur)
        .map(bits_to_priority)
        .map(|v| v as u32)
        .sum();
    log::info!("Diffs for elves = {}", result);
}

/// Day 3 part 1
fn find_uneven_item(bags_string: &str) {
    let sum_of_diffs = bags_string.lines().map(evaluate_rucksack).sum::<u32>();
    log::info!("Sum of differences = {}", sum_of_diffs);
}

fn evaluate_rucksack(bag_string: &str) -> u32 {
    let (compartiment1, compartiment2) = bag_string.halve();

    // compute each side bit repr
    let c1_bitflags = compartiment1.chars().fold(0_u64, |bits, c| {
        set_letter_bitflags(bits, letter_priority(c))
    });
    let c2_bitflags = compartiment2.chars().fold(0_u64, |bits, c| {
        set_letter_bitflags(bits, letter_priority(c))
    });

    let and = c1_bitflags & c2_bitflags;

    let priority_diff = bits_to_priority(and);
    let letter_diff = _bits_to_letter(and);
    log::debug!(
        "{:24} - {:24}   {} -> prio:{:02}  - C1 AND C2 = {:064b}",
        compartiment1,
        compartiment2,
        letter_diff,
        priority_diff,
        and
    );

    priority_diff as u32
}

fn compute_bitflags_for_string(s: &str) -> u64 {
    s.chars().fold(0_u64, |bits, c| {
        set_letter_bitflags(bits, letter_priority(c))
    })
}

/// Returns the letter priority (1-52) as per the day3 notice
fn letter_priority(letter: char) -> u8 {
    let letter = letter as u8;
    const MAJ_OFFSET: u8 = b'A' - 27;
    const MIN_OFFSET: u8 = b'a' - 1;
    match letter {
        maj @ 65..=90 => {
            // A -> 27
            // Z -> 52
            maj - MAJ_OFFSET
        }
        min @ 97..=122 => {
            // a -> 1
            // z -> 26
            min - MIN_OFFSET
        }
        _ => 0, // 2^0
    }
}

/// helper to display a letter from a priority
fn _priority_as_letter(priority: u8) -> char {
    const MAJ_OFFSET: u8 = b'A' - 27;
    const MIN_OFFSET: u8 = b'a' - 1;
    match priority {
        min @ 1..=26 => (min + MIN_OFFSET) as char,
        maj @ 27..=52 => (maj + MAJ_OFFSET) as char,
        _ => '!' as char, // 2^0
    }
}

/// update a u64 with flags corresponding to letters found
fn set_letter_bitflags(mut acc: u64, letter: u8) -> u64 {
    acc |= 1 << letter;
    acc
}

// TODO: sth more efficient
/// Take au64 with only 1 flag raised and return a priority (ie a letter)
fn bits_to_priority(bits: u64) -> u8 {
    for priority in 0_u8..=52 {
        if bits >> priority & 1 == 1 {
            return priority;
        }
    }
    return 0;
}

/// Return a letter from the bit 1 of this u64
fn _bits_to_letter(bits: u64) -> char {
    _priority_as_letter(bits_to_priority(bits))
}

/// Split reference in two
trait Halve {
    fn halve(&self) -> (&Self, &Self);
}

/// Allow use to write let (a, b) = str.halve()
impl Halve for str {
    fn halve(&self) -> (&str, &str) {
        assert!(self.len() % 2 == 0);
        self.split_at(self.len() / 2)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        _priority_as_letter, bits_to_priority, evaluate_rucksack, letter_priority,
        set_letter_bitflags,
    };

    #[test]
    fn letter_priorities_are_valid() {
        assert_eq!(letter_priority('a'), 1);
        assert_eq!(letter_priority('z'), 26);
        assert_eq!(letter_priority('A'), 27);
        assert_eq!(letter_priority('Z'), 52);
        assert_eq!(letter_priority('-'), 0);
    }

    #[test]
    fn bitflag_position_from_letter_priority_and_previous_flags_is_valid() {
        assert_eq!(set_letter_bitflags(0, letter_priority('a')), 2);
        assert_eq!(set_letter_bitflags(0, letter_priority('b')), 4);
        assert_eq!(set_letter_bitflags(2, letter_priority('b')), 6);
        assert_ne!(
            set_letter_bitflags(0, letter_priority('A')),
            set_letter_bitflags(0, letter_priority('z'))
        );
        assert_eq!(
            set_letter_bitflags(1, letter_priority('Z')),
            1 + 2_u64.pow(52)
        );
    }

    #[test]
    fn retrieve_priority_from_u64_with_only_one_flag_set_up() {
        assert_eq!(bits_to_priority(1), 0);
        assert_eq!(bits_to_priority(2), 1);
        assert_eq!(bits_to_priority(4), 2);
    }

    #[test]
    fn retrieve_priority_from_u64_works_until_52() { // 52 => Z
        assert_ne!(bits_to_priority(2_u64.pow(52)), 0);
        assert_eq!(bits_to_priority(2_u64.pow(53)), 0);
    }

    #[test]
    fn convert_priorities_and_letters() {
        assert_eq!(_priority_as_letter(letter_priority('a')), 'a');
        assert_eq!(_priority_as_letter(letter_priority('Z')), 'Z');
        assert_eq!(_priority_as_letter(letter_priority('A')), 'A');
        assert_eq!(_priority_as_letter(letter_priority('z')), 'z');
        assert_eq!(_priority_as_letter(letter_priority(':')), '!');
        assert_eq!(letter_priority(_priority_as_letter(0)), 0);
        assert_eq!(letter_priority(_priority_as_letter(1)), 1);
        assert_eq!(letter_priority(_priority_as_letter(52)), 52);
        assert_eq!(letter_priority(_priority_as_letter(53)), 0);
    }

    #[test]
    fn test_example_data_part1() {
        const INPUT: &str = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let all_diffs: u32 = INPUT.lines().map(evaluate_rucksack).sum();
        assert_eq!(all_diffs, 157);
    }
}
