use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let parse_u32 = |s: &str| {
        s.parse::<u32>()
            .expect(format!("Impossible to parse integer {}", s).as_str())
    };
    let r = read_to_string("./data/day1.dat")?
        .trim()
        .split("\n\n")
        .map(|grpd_lines| grpd_lines.split('\n').map(parse_u32).sum())
        .enumerate()
        .fold(
            (0, 0),
            |max, cur: (usize, u32)| if cur.1 > max.1 { cur } else { max },
        );

    println!("Elf (idx, pickings): {:?}", r);

    Ok(())
}
