use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let data = read_to_string("./data/day6.dat")?;
    match find_message_index(&data, 14) {
        Some((start, end, seq)) => println!(
            "Index of sequence : {:?}\nIndex of message : {:?}\nSequence: {:?}",
            start, end, seq
        ),
        None => println!("Not found"),
    }
    Ok(())
}

fn find_message_index(s: &str, start_seq_size: usize) -> Option<(usize, usize, &[u8])> {
    let sequence_found: Option<(usize, &[u8])> = s
        .as_bytes()
        .windows(start_seq_size)
        .enumerate()
        // .find(|(i, w)| (w[0] != w[1] && w[1] != w[2] && w[2] != w[3] && w[3] != w[0] && w[0] != w[2] && w[1] != w[3]);
        .find(|(_, w)| !has_duplicate(w));
    sequence_found.map(|(seq_idx, seq)| (seq_idx, seq_idx + start_seq_size, seq))
}

fn cross_myself<T>(list: &[T]) -> Vec<(&T, &T)>
where
    T: std::fmt::Debug,
{
    let mut acc = 0;
    for l in 1..=list.len() {
        acc += l - 1;
    }
    let mut vec = Vec::with_capacity(acc);
    for (i, x) in list.iter().enumerate() {
        for (_, y) in list[..i].iter().enumerate() {
            vec.push((x, y));
        }
    }
    vec
}

fn has_duplicate<T>(list: &[T]) -> bool
where
    T: PartialEq + std::fmt::Debug,
{
    cross_myself(list).iter().find(|(x, y)| x == y).is_some()
}

#[cfg(test)]
mod test {
    use crate::{cross_myself, find_message_index, has_duplicate};

    #[test]
    fn validate_has_duplicate_for_w4() {
        const TEST: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let has_no_dup_4_window = |w: &[u8]| {
            w[0] != w[1]
                && w[1] != w[2]
                && w[2] != w[3]
                && w[3] != w[0]
                && w[0] != w[2]
                && w[1] != w[3]
        };

        TEST.as_bytes()
            .windows(4)
            .map(|c| {
                println!("{:?}", c);
                cross_myself(c).iter().for_each(|(l, r)| {
                    println!("{}-{}", l, r);
                });
                c
            })
            .for_each(|window4| assert_eq!(has_duplicate(window4), !has_no_dup_4_window(window4)))
    }

    #[test]
    fn part_1_aoc_4chars() {
        const T1: (&str, usize) = ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5);
        const T2: (&str, usize) = ("nppdvjthqldpwncqszvftbrmjlhg", 6);
        const T3: (&str, usize) = ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10);
        const T4: (&str, usize) = ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11);

        assert_eq!(find_message_index(T1.0, 4).map(|v| v.1), Some(T1.1));
        assert_eq!(find_message_index(T2.0, 4).map(|v| v.1), Some(T2.1));
        assert_eq!(find_message_index(T3.0, 4).map(|v| v.1), Some(T3.1));
        assert_eq!(find_message_index(T4.0, 4).map(|v| v.1), Some(T4.1));
    }

    #[test]
    fn part_2_aoc_14chars() {
        let tests: Vec<(&str, usize)> = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for test in tests {
            assert_eq!(find_message_index(test.0, 14).map(|v| v.1), Some(test.1));
        }
    }
}
