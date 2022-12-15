use std::{fs::read_to_string, num::ParseIntError};

fn debug_mapped_items<I: std::fmt::Debug>(item: I) -> I {
    log::debug!("{:?}", item);
    item
}

fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let sum =
        count_teams_having_complete_taks_inclusion(read_to_string("./data/day4.dat")?.as_str());
    log::info!("Number of team having all tasks of a member included in the tasks list of the other member is : {}", sum);

    Ok(())
}

fn count_teams_having_complete_taks_inclusion(s: &str) -> usize {
    s.lines()
        .map(debug_mapped_items)
        .map(|line| ElvesTeam::parse(line).expect("failed to parse line"))
        .map(debug_mapped_items)
        .map(|team| if team.has_assign_inclusion() { 1 } else { 0 })
        .map(debug_mapped_items)
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

    fn has_assign_inclusion(self) -> bool {
        self.members.iter().enumerate().fold(false, |acc, (i, left)| {
            acc || self
                .members
                .iter()
                .enumerate()
                .fold(false, |acc, (j, right)| {
                    if i == j {
                        acc
                    } else {
                        let res = acc || left.includes(right);
                        log::debug!("Does {:?} includes {:?} ? The anwser is {}", left, right, left.includes(right));
                        res
                    }
                })
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{count_teams_having_complete_taks_inclusion, Assignment, ElvesTeam};

    #[test]
    fn parse_team() {
        assert_eq!(
            ElvesTeam::parse("1-4,2-6").unwrap(),
            ElvesTeam {
                members: vec![Assignment(1, 4), Assignment(2, 6)]
            }
        );
    }

    #[test]
    fn validate_with_example() {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
        const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let sum = count_teams_having_complete_taks_inclusion(INPUT);
        assert_eq!(sum, 2);
    }
}
