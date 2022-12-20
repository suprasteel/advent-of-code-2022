use std::{borrow::Cow, fmt::Display, str::FromStr};

use crate::{
    charables::{ToChar, TryFromChar},
    parse::instruction::Instruction,
};

pub type Stack<T> = Vec<T>;

pub struct Ship<T> {
    internal: Vec<Stack<T>>,
}

impl<T> FromStr for Ship<T>
where
    T: TryFromChar + Clone,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ship::parse(s)
            .map(|(_, ship)| ship)
            .map_err(|s| s.to_string())
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
        assert!(
            self.internal.len() > pos,
            "len = {}, pushing in [{}]",
            self.internal.len(),
            pos
        );
        self.internal[pos].push(value);
        self
    }

    pub fn execute(&mut self, inst: Instruction) {
        let machine_idx = |human_idx| human_idx - 1;
        /* day5 part 1
        for _ in 0..inst.by {
            let pop = self.internal[machine_idx(inst.from)].pop();
            pop.map(|v| self.internal[machine_idx(inst.to)].push(v));
        }
        */
        let src_len = self.internal[machine_idx(inst.from)].len();
        assert!(
            src_len >= inst.by,
            "Crane 9001 trying to lift {} whereas there are only {}",
            inst.by,
            src_len
        );
        let poped_vec = self.internal[machine_idx(inst.from)].split_off(src_len - inst.by);
        poped_vec
            .into_iter()
            .for_each(|elt| self.internal[machine_idx(inst.to)].push(elt));
    }
}

impl<T> Ship<T>
where
    T: ToChar,
{
    pub fn tops_values_as_string(&self) -> String {
        self.internal
            .iter()
            .map(|vector| vector.last().map(T::to_char))
            .filter(Option::is_some)
            .fold("".to_string(), |acc, cur| {
                format!("{}{}", acc, cur.unwrap())
            })
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
            Some(c) => Cow::from(
                ['[', <T as ToChar>::to_char(c), ']']
                    .into_iter()
                    .collect::<String>(),
            ),
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
