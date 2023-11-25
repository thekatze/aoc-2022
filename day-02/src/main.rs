const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for GameResult {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Win),
            _ => Err("Invalid choice"),
        }
    }
}
impl TryFrom<&str> for Choice {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err("Invalid choice"),
        }
    }
}

#[derive(Debug)]
struct Game {
    opponent_move: Choice,
    my_move: Choice,
    result: GameResult,
}

#[derive(Debug)]
enum GameResult {
    Win,
    Tie,
    Loss,
}

impl Choice {
    fn from_opponent_and_result(opponent_move: &Choice, result: &GameResult) -> Self {
        match (opponent_move, result) {
            (Choice::Rock, GameResult::Win) => Choice::Paper,
            (Choice::Rock, GameResult::Tie) => Choice::Rock,
            (Choice::Rock, GameResult::Loss) => Choice::Scissors,
            (Choice::Paper, GameResult::Win) => Choice::Scissors,
            (Choice::Paper, GameResult::Tie) => Choice::Paper,
            (Choice::Paper, GameResult::Loss) => Choice::Rock,
            (Choice::Scissors, GameResult::Win) => Choice::Rock,
            (Choice::Scissors, GameResult::Tie) => Choice::Scissors,
            (Choice::Scissors, GameResult::Loss) => Choice::Paper,
        }
    }
}

impl TryFrom<&str> for Game {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (oppenent_move, result) = value.split_once(' ').ok_or("couldnt split")?;

        let opponent_move = Choice::try_from(oppenent_move)?;
        let result = GameResult::try_from(result)?;

        let my_move = Choice::from_opponent_and_result(&opponent_move, &result);

        Ok(Self {
            opponent_move,
            my_move,
            result,
        })
    }
}

impl Game {
    fn score(&self) -> u32 {
        let result_score = match self.result {
            GameResult::Win => 6,
            GameResult::Tie => 3,
            GameResult::Loss => 0,
        };

        let choice_score = match self.my_move {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };

        result_score + choice_score
    }
}

fn main() {
    let score = INPUT
        .lines()
        .filter_map(|game| Game::try_from(game).map(|game| game.score()).ok())
        .sum::<u32>();

    dbg!(score);
}
