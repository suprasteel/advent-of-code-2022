use crate::ship::Ship;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Char(char),
    Index(u8),
    Empty,
}

impl Cell {
    fn extract_char(&self) -> Option<char> {
        match self {
            Self::Char(c) => Some(*c),
            _ => None,
        }
    }

    fn is_index(&self) -> bool {
        matches!(self, Cell::Index(_))
    }

    fn parse<'s>(s: &'s str) -> Result<(&'s str, Cell), &'s str> {
        log::debug!("parse cell : {} ... ({})", &s[0..3], s.len());
        if s.len() < 3 {
            Err(s)
        } else {
            let (cell, rem) = s.split_at(3);
            let mut chrs = cell.chars();
            let c1 = chrs.next().unwrap();
            let c2 = chrs.next().unwrap();
            let c3 = chrs.next().unwrap();
            match (c1, c2, c3) {
                (' ', ' ', ' ') => Ok((rem, Cell::Empty)),
                ('[', c, ']') => Ok((rem, Cell::Char(c))),
                (' ', n, ' ') if n.is_numeric() => Ok((rem, Cell::Index(n as u8 - '0' as u8))),
                _ => Err(cell),
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Line {
    Content(Vec<Cell>),
    Abscissa(Vec<Cell>),
    Empty,
}

impl Line {
    fn parse<'l>(input: &'l str) -> Result<(&'l str, Line), &'l str> {
        let (mut line, rest) = input.split_once('\n').unwrap_or((input, ""));
        log::debug!("parse line : {} ... ({})", line, line.len());
        match line.len() {
            0 => Ok((&rest, Line::Empty)),
            n if (n + 1) % 3 == 0 => {
                let mut cells = Vec::with_capacity(n);
                let mut line_of_indexes: Option<bool> = None;
                loop {
                    let cell: Cell;
                    (line, cell) = Cell::parse(line)?;

                    log::trace!(target: "line", "cell {:?}", cell);

                    if !check_consistency(&mut line_of_indexes, cell.is_index()) {
                        return Err(" both indexes and values found");
                    }

                    cells.push(cell);

                    match line.chars().nth(0) {
                        Some(' ') => {
                            log::trace!(target: "line", "continue with <{}>", line);
                            line = &line[1..];
                        }
                        _ => {
                            break;
                        }
                    }
                }
                Ok((
                    rest,
                    if line_of_indexes.unwrap() {
                        Line::Abscissa(cells)
                    } else {
                        Line::Content(cells)
                    },
                ))
            }
            _ => {
                log::error!("Malformed line of len={} : {} ", line.len(), line);
                Err("malformed line")
            }
        }
    }
}

/// check that given value equals the first value passed
fn check_consistency<T>(base: &mut Option<T>, next_val: T) -> bool
where
    T: PartialEq + Copy,
{
    base.get_or_insert(next_val) == &next_val
}

pub fn parse_tank<'s>(s: &'s str) -> Result<Ship<char>, &'s str> {
    log::debug!("parse tank : {} ... ({})", s, s.len());
    //let width = None;
    let mut rest = s;
    let mut content = vec![];
    let mut width = None;
    loop {
        let cells: Line;
        (rest, cells) = Line::parse(rest)?;
        match cells {
            Line::Content(c) => {
                log::debug!("parse tank : content found");
                if !check_consistency(&mut width, c.len()) {
                    return Err("Varrying width of ship stacks detected :(");
                }
                let chars = c
                    .iter()
                    .map(Cell::extract_char)
                    .collect::<Vec<Option<char>>>();
                content.push(chars);
            }
            Line::Abscissa(indexes) => {
                log::debug!("parse tank : indexes found");
                assert_eq!(indexes.len(), width.unwrap());
                assert!(indexes
                    .iter()
                    .enumerate()
                    .map(|(i, v)| v == &Cell::Index((i + 1) as u8))
                    .fold(true, |acc, ok| acc && ok));
                break;
            }
            _ => {
                log::error!("Parsing ship tank failed on cell {:?}", cells);
                return Err("Line Cells error")
            },
        }
    }
    let width = width.unwrap();

    let mut ship = Ship::new_empty_ship(width);

    content.reverse();
    content.iter().for_each(|vector| {
        log::debug!("{:?}", vector);
        for i in 0..width {
            vector[i].map(|value| ship.push_at_top_of_stack(i, value));
        }
    });
    // ship tank parsed
    Ok(ship)
}
