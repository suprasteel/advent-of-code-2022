mod charables;
mod parse;
mod ship;

#[cfg(test)]
mod test;

use std::fs::read_to_string;

use ship::Ship;

use crate::parse::{instruction::Instruction, skip_whitespace};

fn main() -> std::io::Result<()> {
    let data_string = read_to_string("./data/day5.dat")?;
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let (rest, mut ship) = Ship::<char>::parse(&data_string).expect("parsing err");
    dbg!(ship.to_string());

    let mut rest = skip_whitespace(rest);

    dbg!(rest);
    let mut instructions = vec![];
    while !rest.is_empty() {
        let instruction;

        (rest, instruction) = Instruction::parse(rest).expect(":(");
        instructions.push(instruction);
        if rest.len() > 20 {
            log::info!("rest (20chars...) : {}", &rest[..20]);
        } else {
            log::info!("rest  : {}", &rest);
        }
        rest = skip_whitespace(rest);
        if rest.len() > 20 {
            log::info!("rest (20chars...) : {}", &rest[..20]);
        } else {
            log::info!("rest  : {}", &rest);
        }
    }

    for instruction in instructions {
        dbg!(&instruction);
        ship.execute(instruction);
    }

    println!("{}", ship);
    println!("{}", ship.tops_values_as_string());

    Ok(())
}
