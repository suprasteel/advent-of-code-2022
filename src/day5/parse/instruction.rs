#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub(crate) by: usize,
    pub(crate) from: usize,
    pub(crate) to: usize,
}

#[derive(Debug)]
enum Action {
    Move,
}

impl Action {
    fn parse<'s>(input: &'s str) -> Result<(&'s str, Self), String> {
        let (line, rest) = input.split_once(' ').unwrap_or((input, ""));
        match line {
            "move" => Ok((rest, Action::Move)),
            _ => Err(format!("unknown action <{}>", line)),
        }
    }
}

#[derive(Debug)]
enum Dir {
    From,
    To,
}

impl Dir {
    fn parse<'s>(input: &'s str) -> Result<(&'s str, Self), String> {
        let (word, rest) = input.split_once(' ').unwrap_or((input, ""));
        match word {
            "from" => Ok((rest, Self::From)),
            "to" => Ok((rest, Self::To)),
            _ => Err("unknown direction".to_string()),
        }
    }
}

pub fn parse_usize<'s>(input: &'s str) -> Result<(&'s str, usize), String> {
    parse_u64(input).map(|(rest, value)| (rest, value as usize))
}

fn parse_u64<'s>(input: &'s str) -> Result<(&'s str, u64), String> {
    let (nb_str_len, value) = input
        .chars()
        .take_while(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).expect("invalid digit") as u64)
        .enumerate()
        .fold((0_usize, 0_u64), |(_, total), (index, digit)| {
            (index + 1, total * 10 + digit)
        });
    if input.chars().nth(nb_str_len) == Some(' ') {
        Ok((&input[(nb_str_len + 1)..], value))
    } else {
        Ok((&input[nb_str_len..], value))
    }
}

impl Instruction {
    pub fn new(repeat: usize, from: usize, to: usize) -> Self {
        Self {
            by: repeat,
            from,
            to,
        }
    }

    pub fn parse<'s>(input: &'s str) -> Result<(&'s str, Self), String> {
        let mut from = None;
        let mut to = None;
        let mut set_dir_val = |dir, val| {
            if matches!(dir, Dir::From) {
                from = Some(val);
            } else {
                to = Some(val);
            }
        };

        let (rest, _) = Action::parse(input)?;
        let (rest, qty) = parse_usize(rest)?;
        let (rest, dir) = Dir::parse(rest)?;
        let (rest, value) = parse_usize(rest)?;
        set_dir_val(dir, value);
        let (rest, dir) = Dir::parse(rest)?;
        let (rest, value) = parse_usize(rest)?;
        set_dir_val(dir, value);

        let from = from.ok_or("no source value provided".to_string())?;
        let to = to.ok_or("no destination value provided".to_string())?;

        Ok((rest, Instruction::new(qty, from, to)))
    }
}
