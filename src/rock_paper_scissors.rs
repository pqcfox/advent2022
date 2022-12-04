#[derive(Debug, Clone, Copy, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum GameResult {
    Win,
    Loss,
    Tie,
}

pub fn run(input: &str) {
    let mut first_total: u32 = 0;
    let mut second_total: u32 = 0;
    for line in input.lines() {
        // parse the letters and the enemy choice
        let my_letter = line.chars().nth(2).expect("Failed to get my choice");
        let enemy_letter = line.chars().nth(0).expect("Failed to get enemy choice");
        let enemy_choice = match enemy_letter {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            'C' => Choice::Scissors,
            _ => panic!("Invalid enemy choice"),
        };

        // in the first strategy, get my choice and the result
        let first_my_choice = match my_letter {
            'X' => Choice::Rock,
            'Y' => Choice::Paper,
            'Z' => Choice::Scissors,
            _ => panic!("Invalid self/result choice"),
        };

        let first_game_result = match (&first_my_choice, &enemy_choice) {
            (a, b) if a == b => GameResult::Tie,
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => GameResult::Win,
            _ => GameResult::Loss,
        };

        // in the second strategy, get my choice and the result
        let second_game_result = match my_letter {
            'X' => GameResult::Loss,
            'Y' => GameResult::Tie,
            'Z' => GameResult::Win,
            _ => panic!("Invalid self/result choice"),
        };

        let second_my_choice = match (&enemy_choice, &second_game_result) {
            (&c, GameResult::Tie) => c,
            (Choice::Scissors, GameResult::Win) | (Choice::Paper, GameResult::Loss) => Choice::Rock,
            (Choice::Rock, GameResult::Win) | (Choice::Scissors, GameResult::Loss) => Choice::Paper,
            (Choice::Paper, GameResult::Win) | (Choice::Rock, GameResult::Loss) => Choice::Scissors,
        };

        // update both total scores
        first_total += score_round(first_my_choice, first_game_result);
        second_total += score_round(second_my_choice, second_game_result);
    }

    println!("Total score for first interpretation: {}", first_total);
    println!("Total score for second interpretation: {}", second_total);
}

fn score_round(my_choice: Choice, game_result: GameResult) -> u32 {
    let mut score: u32 = 0;

    score += match my_choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    score += match game_result {
        GameResult::Win => 6,
        GameResult::Tie => 3,
        GameResult::Loss => 0,
    };

    score
}
