use std::fs::read_to_string;

/// For example, suppose you were given the following strategy guide:
///
/// A Y
/// B X
/// C Z
///
/// This strategy guide predicts and recommends the following:
///
///     In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
///     In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
///     The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
///
/// In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
///
/// What would your total score be if everything goes exactly according to your strategy guide?
///
/// 1 ROCK       A   X
/// 2 PAPER      B   Y
/// 3 SCISSORS   C   Z
///
fn main() -> std::io::Result<()> {
    let data_string = read_to_string("./data/day2.dat")?;
    let result = rock_paper_scissors_sum(data_string);
    println!("{}", result);
    Ok(())
}

fn calc_points(s: &str) -> u32 {
    match s {
        // A is rock
        "A X" => 4, // 1 (rock)     + 3 (draw)
        "A Y" => 8, // 2 (paper)    + 6 (victory)
        "A Z" => 3, // 3 (scissors) + 0 (loss)
        // B is paper
        "B X" => 1, // 1 (rock)     + 0 (loss)
        "B Y" => 5, // 2 (paper)    + 3 (draw)
        "B Z" => 9, // 3 (scissors) + 6 (victory)
        // C is scissors
        "C X" => 7, // 1 (rock)     + 6 (victory)
        "C Y" => 2, // 2 (paper)    + 0 (loss)
        "C Z" => 6, // 3 (scissors) + 3 (draw)
        _ => panic!(),
    }
}

fn rock_paper_scissors_sum(string: String) -> u32 {
    string.trim().lines().map(calc_points).sum()
}

#[test]
fn test_example() {
    const INPUT: &str = "
A Y
B X
C Z
";
    let sum = rock_paper_scissors_sum(INPUT.to_string());
    assert_eq!(sum, 15);
}

