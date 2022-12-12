use std::{fs::read_to_string};

fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let bags_string = read_to_string("./data/day3.dat")?;
    let sum_of_diffs = bags_string.lines().map(evaluate_rucksack).sum::<u32>();

    log::info!("Sum of differences = {}", sum_of_diffs);

    Ok(())
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
    let letter_diff = bits_to_letter(and);
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

fn priority_as_letter(priority: u8) -> char {
    const MAJ_OFFSET: u8 = b'A' - 27;
    const MIN_OFFSET: u8 = b'a' - 1;
    match priority {
        min @ 1..=26 => (min + MIN_OFFSET) as char,
        maj @ 27..=52 => (maj + MAJ_OFFSET) as char,
        _ => '!' as char, // 2^0
    }
}

fn set_letter_bitflags(mut acc: u64, letter: u8) -> u64 {
    acc |= 1 << letter;
    acc
}

fn bits_to_priority(bits: u64) -> u8 {
    for priority in 0_u8..=52 {
        if bits >> priority & 1 == 1 {
            return priority;
        }
    }
    return 0;
}

fn bits_to_letter(bits: u64) -> char {
    priority_as_letter(bits_to_priority(bits))
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
        bits_to_priority, evaluate_rucksack, letter_priority, priority_as_letter,
        set_letter_bitflags,
    };

    #[test]
    fn test_letter_priority() {
        assert_eq!(letter_priority('a'), 1);
        assert_eq!(letter_priority('z'), 26);
        assert_eq!(letter_priority('A'), 27);
        assert_eq!(letter_priority('Z'), 52);
    }

    #[test]
    fn test_set_bitflags() {
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
    fn test_bits_to_priority() {
        assert_eq!(bits_to_priority(1), 0);
        assert_eq!(bits_to_priority(2), 1);
        assert_eq!(bits_to_priority(4), 2);
        assert_ne!(bits_to_priority(2_u64.pow(52)), 0);
        assert_eq!(bits_to_priority(2_u64.pow(53)), 0);
    }

    #[test]
    fn test_priority_as_letter() {
        assert_eq!(priority_as_letter(letter_priority('a')), 'a');
        assert_eq!(priority_as_letter(letter_priority('Z')), 'Z');
        assert_eq!(priority_as_letter(letter_priority('A')), 'A');
        assert_eq!(priority_as_letter(letter_priority('z')), 'z');
        assert_eq!(priority_as_letter(letter_priority(':')), '!');
        assert_eq!(letter_priority(priority_as_letter(0)), 0);
        assert_eq!(letter_priority(priority_as_letter(1)), 1);
        assert_eq!(letter_priority(priority_as_letter(52)), 52);
        assert_eq!(letter_priority(priority_as_letter(53)), 0);
    }

    #[test]
    fn test_example() {
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
