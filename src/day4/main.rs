use std::{fs::read_to_string, num::ParseIntError};

const VARIANT: u8 = 2;
const LOG_LVL: log::Level = log::Level::Trace;

const fn get_code_variant() -> fn(&str) -> usize {
    match VARIANT {
        1 => count_teams_having_complete_taks_inclusion,
        2 => count_teams_having_overlaping_tasks,
        _ => panic!("not a valid variant"),
    }
}

fn trace_items<I: std::fmt::Debug>(item: I) -> I {
    log::trace!("{:?}: {:?}", std::any::type_name::<I>(), item);
    item
}

fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(LOG_LVL).unwrap();
    log::info!("Running variant {} with log level {:?}", VARIANT, LOG_LVL);
    let string = read_to_string("./data/day4.dat")?;
    let c = string.lines().count();
    let sum = get_code_variant()(string.trim());
    log::info!("Result:  {} ({} lines)", sum, c);
    Ok(())
}

fn count_teams_having_complete_taks_inclusion(s: &str) -> usize {
    s.lines()
        .map(trace_items)
        .map(|line| ElvesTeam::parse(line).expect("failed to parse line"))
        .map(trace_items)
        .map(|team| if team.has_assign_inclusion() { 1 } else { 0 })
        .map(trace_items)
        .sum::<usize>()
}

fn count_teams_having_overlaping_tasks(s: &str) -> usize {
    s.lines()
        .map(trace_items)
        .map(|line| ElvesTeam::parse(line).expect("failed to parse line"))
        .map(trace_items)
        .map(|team| if team.has_assign_overlaps() { 1 } else { 0 })
        .map(trace_items)
        .sum::<usize>()
}

type Rslt<A> = std::result::Result<A, Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq)]
struct Assignment(usize, usize);

impl Assignment {
    fn parse(s: &str) -> Rslt<Self> {
        let parts = s
            .split("-")
            .map(|n| n.parse::<usize>())
            .collect::<Result<Vec<usize>, ParseIntError>>()?;
        assert_eq!(parts.len(), 2);
        Ok(Assignment(parts[0], parts[1]))
    }

    fn includes(&self, other: &Assignment) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        if other.0 > self.0 {
            other.0 <= self.1
        } else {
            other.1 >= self.0
        }
    }
}

#[derive(Debug, PartialEq)]
struct ElvesTeam {
    members: Vec<Assignment>,
}

impl ElvesTeam {
    fn parse(s: &str) -> Rslt<Self> {
        Ok(Self {
            members: s
                .split(",")
                .map(|a| Assignment::parse(a))
                .collect::<Rslt<Vec<Assignment>>>()?,
        })
    }

    // useless cross product here -> TODO: take only the triangle
    fn has_assign_overlaps(self) -> bool {
        self.members
            .iter()
            .enumerate()
            .fold(false, |acc, (i, left)| {
                acc || self
                    .members
                    .iter()
                    .enumerate()
                    .fold(false, |acc, (j, right)| {
                        if i == j {
                            acc
                        } else {
                            let overlaps = left.overlaps(right);
                            log::debug!(
                                "Does {:?} overlaps {:?} ? The anwser is {}",
                                left,
                                right,
                                overlaps
                            );
                            acc || overlaps
                        }
                    })
            })
    }

    fn has_assign_inclusion(self) -> bool {
        self.members
            .iter()
            .enumerate()
            .fold(false, |acc, (i, left)| {
                acc || self
                    .members
                    .iter()
                    .enumerate()
                    .fold(false, |acc, (j, right)| {
                        if i == j {
                            acc
                        } else {
                            let includes = left.includes(right);
                            log::debug!(
                                "Does {:?} includes {:?} ? The anwser is {}",
                                left,
                                right,
                                includes
                            );
                            acc || includes
                        }
                    })
            })
    }
}

#[cfg(test)]
mod test;
