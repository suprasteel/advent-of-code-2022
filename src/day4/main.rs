use std::{fs::read_to_string, num::ParseIntError};

const VARIANT: u8 = 2;

const fn get_code_variant() -> fn(&str) -> usize {
    match VARIANT {
        1 => count_teams_having_complete_taks_inclusion,
        2 => count_teams_having_overlaping_tasks,
        _ => panic!("not a valid variant"),
    }
}

fn debug_mapped_items<I: std::fmt::Debug>(item: I) -> I {
    log::debug!("{:?}", item);
    item
}

fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let string = read_to_string("./data/day4.dat")?;
    let c = string.lines().count();
    let sum = get_code_variant()(string.as_str());
    log::info!("Result:  {} ({} lines)", sum, c);


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

fn count_teams_having_overlaping_tasks(s: &str) -> usize {
    s.lines()
        .map(debug_mapped_items)
        .map(|line| ElvesTeam::parse(line).expect("failed to parse line"))
        .map(debug_mapped_items)
        .map(|team| if team.has_assign_overlaps() { 1 } else { 0 })
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

    fn overlaps(&self, other: &Assignment) -> bool {
        self.0 <= other.0 && self.1 >= other.0 || self.1 >= other.0 && self.1 <= self.0
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
                            let res = acc || left.overlaps(right);
                            log::debug!(
                                "Does {:?} overlaps {:?} ? The anwser is {}",
                                left,
                                right,
                                left.overlaps(right)
                            );
                            res
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
                            let res = acc || left.includes(right);
                            log::debug!(
                                "Does {:?} includes {:?} ? The anwser is {}",
                                left,
                                right,
                                left.includes(right)
                            );
                            res
                        }
                    })
            })
    }
}

#[cfg(test)]
mod test {
    use crate::{count_teams_having_complete_taks_inclusion, Assignment, ElvesTeam, count_teams_having_overlaping_tasks};

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
    fn the_same_task_overlaps() {
        assert_eq!(
            ElvesTeam::parse("1-1,1-1").unwrap().has_assign_overlaps(),
            true
        );
        
    }

    #[test]
    fn the_same_tasks_overlaps() {
        assert_eq!(
            ElvesTeam::parse("1-2,1-2").unwrap().has_assign_overlaps(),
            true
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

    #[test]
    fn validate_with_example_2() {
        //simple_logger::init_with_level(log::Level::Debug).unwrap();

        const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let sum = count_teams_having_overlaping_tasks(INPUT);
        assert_eq!(sum, 4);
    }

    #[test]
    fn validate_with_example_2_2() {
        //simple_logger::init_with_level(log::Level::Debug).unwrap();

        const INPUT: &str = "1-3,4-6
1-3,100-200
5-60,60-100
1-1,1-1
2-2,4-100
89-94,32-89
40-77,77-77
28-70,28-70
0-0,0-0";
        let sum = count_teams_having_overlaping_tasks(INPUT);
        assert_eq!(sum, 5);
    }
}
