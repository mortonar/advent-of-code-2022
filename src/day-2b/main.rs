use std::io;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let total_score: u64 = io::stdin()
        .lines()
        .map(|line| {
            let line: String = line.unwrap();
            let mut choices = line.trim().split_ascii_whitespace();
            let opponent = Choice::from_str(choices.next().unwrap()).unwrap();
            let end = RoundEnd::from_str(choices.next().unwrap()).unwrap();
            opponent.score(&end)
        })
        .sum();
    println!("{}", total_score);
    Ok(())
}

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

enum RoundEnd {
    Lose,
    Draw,
    Win,
}

impl FromStr for RoundEnd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "X" => Ok(RoundEnd::Lose),
            "Y" => Ok(RoundEnd::Draw),
            "Z" => Ok(RoundEnd::Win),
            _ => Err(()),
        }
    }
}

// ROCK     (A) (X)  1 Beats: (C) (Z)
// PAPER    (B) (Y)  2 Beats: (A) (X)
// SCISSORS (C) (Z)  3 Beats: (B) (Y)
// lost 0, draw 3, won 6
// Lose X
// Draw Y
// Win  Z
impl Choice {
    fn score(&self, end: &RoundEnd) -> u64 {
        match self {
            Choice::Rock => match end {
                RoundEnd::Draw => 3 + 1,
                RoundEnd::Lose => 0 + 3,
                RoundEnd::Win => 6 + 2,
            },
            Choice::Paper => match end {
                RoundEnd::Draw => 3 + 2,
                RoundEnd::Lose => 0 + 1,
                RoundEnd::Win => 6 + 3,
            },
            Choice::Scissors => match end {
                RoundEnd::Draw => 3 + 3,
                RoundEnd::Lose => 0 + 2,
                RoundEnd::Win => 6 + 1,
            },
        }
    }
}
