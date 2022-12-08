use std::io;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let total_score: u64 = io::stdin()
        .lines()
        .map(|line| {
            let line: String = line.unwrap();
            let choices: Vec<Choice> = line
                .trim()
                .split_ascii_whitespace()
                .filter_map(|c| Choice::from_str(c).ok())
                .collect();
            let you = &choices[1];
            you.score(&choices[0])
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
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

// ROCK     (A) (X)  1 Beats: (C) (Z)
// PAPER    (B) (Y)  2 Beats: (A) (X)
// SCISSORS (C) (Z)  3 Beats: (B) (Y)
// lost 0, draw 3, won 6
impl Choice {
    fn score(&self, opponent: &Choice) -> u64 {
        match self {
            Choice::Rock => {
                1 + match opponent {
                    Choice::Rock => 3,
                    Choice::Paper => 0,
                    Choice::Scissors => 6,
                }
            }
            Choice::Paper => {
                2 + match opponent {
                    Choice::Rock => 6,
                    Choice::Paper => 3,
                    Choice::Scissors => 0,
                }
            }
            Choice::Scissors => {
                3 + match opponent {
                    Choice::Rock => 0,
                    Choice::Paper => 6,
                    Choice::Scissors => 3,
                }
            }
        }
    }
}
