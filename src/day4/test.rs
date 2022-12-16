use crate::{
    count_teams_having_complete_taks_inclusion, count_teams_having_overlaping_tasks, Assignment,
    ElvesTeam,
};

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
4-6,1-3
100-200,1-3
60-100,5-60
1-1,1-1
4-100,2-2
32-89,89-94
77-77,40-77
28-70,28-70
4-24,2-4";
    let sum = count_teams_having_overlaping_tasks(INPUT);
    assert_eq!(sum, 11);
}
