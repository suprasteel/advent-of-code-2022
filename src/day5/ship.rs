use std::{borrow::Cow, fmt::Display, str::FromStr};

use crate::parse::ship::parse_tank;

pub type Stack<T> = Vec<T>;

pub struct Ship<T> {
    internal: Vec<Stack<T>>,
}

impl<T> Ship<T> {
    pub fn parse_tank(s: &str) -> Result<Ship<char>, &str> {
        parse_tank(s)
    }
}

impl FromStr for Ship<char> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_tank(s).map_err(|s| s.to_string())
    }
}

impl<T> Ship<T> {
    pub fn new_empty_ship(container_stack_width: usize) -> Self {
        let mut ship = Self {
            internal: Vec::with_capacity(container_stack_width),
        };
        for _ in 0..container_stack_width {
            ship.internal.push(vec![])
        }
        ship
    }

    pub fn push_at_top_of_stack(&mut self, pos: usize, value: T) -> &mut Self {
        assert!(self.internal.len() > pos);
        self.internal[pos].push(value);
        self
    }
}

trait ToChar {
    fn to_char(&self) -> char;
}

impl ToChar for char {
    fn to_char(&self) -> char {
        *self
    }
}

impl ToChar for usize {
    fn to_char(&self) -> char {
        match *self {
            0..=9 => (('0' as u32 + *self as u32) as u8) as char,
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            _ => '+'
        }
    }
}

impl<T> Display for Ship<T>
where
    T: ToChar,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let highest_stack_size = |ship: &Self| ship.internal.iter().map(|s| s.len()).max();
        let Some(height) = highest_stack_size(self) else { return write!(f, "\n - the ship is empty - \n"); };
        let width = self.internal.len();

        let get_repr_of = |v: &Vec<T>, i| match v.get(i) {
            Some(c) => Cow::from(['[', <T as ToChar>::to_char(c), ']'].into_iter().collect::<String>()),
            None => Cow::from("   "),
        };

        let mut h: usize = 0;
        let mut output: Vec<String> = Vec::with_capacity(height);
        output.push(
            (1..=width)
                .map(|n| format!(" {} ", n))
                .collect::<Vec<_>>()
                .join(" "), //"
        );

        while h < height {
            let mut line = String::with_capacity((3 * width) + (width - 1));
            let mut w = 0;
            while w < width {
                if w > 0 {
                    line.push(' ')
                }
                line.push_str(get_repr_of(&self.internal[w], h).as_ref());
                w += 1;
            }
            output.push(line);
            h += 1;
        }
        output.reverse();
        write!(f, "{}", output.join("\n"))
    }
}
