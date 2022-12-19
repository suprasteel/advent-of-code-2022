use crate::Ship;

#[rustfmt::skip]
const CONTENT_NUMBER_EXAMPLE_STR: &str = r"
[4]     [6]
[3]     [7]
[2] [2] [8]
[1] [2] [9]
 1   2   3 ";

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
    let cargo = Ship::<char>::parse_tank(&CONTENT_NUMBER_EXAMPLE_STR[1..]).expect("parsing err");

    print!("{}", cargo);

    assert_eq!(cargo.to_string(), &CONTENT_NUMBER_EXAMPLE_STR[1..]);
}
