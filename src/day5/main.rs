mod parse;
mod ship;

use ship::Ship;

#[rustfmt::skip]
const CONTENT_EXAMPLE_STR: &str = r"
[4]     [6]
[3]     [7]
[2] [2] [8]
[1] [2] [9]
 1   2   3 ";

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let ship = Ship::<char>::parse_tank(&CONTENT_EXAMPLE_STR[1..]).expect("parsing err");
    assert_eq!(ship.to_string(), CONTENT_EXAMPLE_STR[1..]);
}


/*
struct Crane;
enum _Cmd {
    Move(MoveCmd),
    Unload(UnloadCmd),
    Load(LoadCmd),
}

struct MoveCmd {
    from: usize,
    to: usize,
}

struct UnloadCmd {
    from: usize,
}

struct LoadCmd {
    to: usize,
    value: usize, // comment mettre un T sans imposer un generic à Cmd ?
}

trait Parse {
    type Output;
    type Error;
    fn parse<'input>(text: &'input str) -> Result<(Self::Output, &'input str), Self::Error>;
}

impl Parse for Crane {
    type Output = _Cmd;

    type Error = usize;

    fn parse<'input>(text: &'input str) -> Result<(Self::Output, &'input str), Self::Error> {
        todo!()
    }
}
*/

// move 1 from 2 to 1

#[cfg(test)]
mod test;
