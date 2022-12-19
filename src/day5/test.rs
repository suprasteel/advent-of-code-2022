use crate::{
    parse::{
        instruction::{parse_usize, Instruction},
        skip_whitespace,
    },
    Ship,
};

#[rustfmt::skip]
const CONTENT_NUMBER_EXAMPLE_STR: &str = r"
[4]     [6]
[3]     [7]
[2] [2] [8]
[1] [2] [9]
 1   2   3 ";

#[rustfmt::skip]
const INST_CONTENT_EXAMPLE_STR: &str = r"
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[test]
fn display_empty_ship() {
    let ship: Ship<char> = Ship::new_empty_ship(0);
    assert_eq!(ship.to_string(), "\n - the ship is empty - \n");
}

#[test]
fn display_ship_content() {
    let ship = {
        let mut ship = Ship::new_empty_ship(3);
        ship.push_at_top_of_stack(0, 1);
        ship.push_at_top_of_stack(0, 2);
        ship.push_at_top_of_stack(0, 3);
        ship.push_at_top_of_stack(0, 4);
        ship.push_at_top_of_stack(1, 2);
        ship.push_at_top_of_stack(1, 2);
        ship.push_at_top_of_stack(2, 9);
        ship.push_at_top_of_stack(2, 8);
        ship.push_at_top_of_stack(2, 7);
        ship.push_at_top_of_stack(2, 6);
        ship
    };

    // remove first <CR> from exemple string
    assert_eq!(ship.to_string(), &CONTENT_NUMBER_EXAMPLE_STR[1..]);
}

#[test]
fn parse_ship() {
    let (_, ship) = Ship::<char>::parse(&CONTENT_NUMBER_EXAMPLE_STR[1..]).expect("parsing err");
    print!("{}", ship);
    assert_eq!(ship.to_string(), &CONTENT_NUMBER_EXAMPLE_STR[1..]);
}

#[test]
fn execute() {
    let (_, mut ship) = Ship::<char>::parse(&CONTENT_NUMBER_EXAMPLE_STR[1..]).expect("parsing err");
    let mut rest = INST_CONTENT_EXAMPLE_STR;
    let mut instructions = vec![];
    rest = skip_whitespace(rest);
    while !rest.is_empty() {
        let instruction;

        (rest, instruction) = Instruction::parse(rest).expect(":(");
        instructions.push(instruction);
        rest = skip_whitespace(rest);
    }

    for instruction in instructions {
        dbg!(&instruction);
        ship.execute(instruction);
    }
}

#[test]
fn parse_instruction() {
    const DATA: &'static str = "move 100 from 1 to 9";
    let (rest, instruction) = Instruction::parse(DATA).unwrap();
    assert!(rest.is_empty());
    assert_eq!(Instruction::new(100, 1, 9), instruction);
}

#[test]
fn parse_number() {
    const DATA: &'static str = "999junk";
    let (rest, number) = parse_usize(DATA).unwrap();
    assert_eq!(rest, "junk");
    assert_eq!(number, 999);
    const DATA2: &'static str = "90563";
    let (rest, number) = parse_usize(DATA2).unwrap();
    assert_eq!(rest, "");
    assert_eq!(number, 90563);
}
