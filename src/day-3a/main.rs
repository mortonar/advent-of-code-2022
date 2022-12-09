use std::collections::HashSet;
use std::io;

fn main() -> anyhow::Result<()> {
    let mut sum: u64 = 0;
    for line in io::stdin().lines() {
        let line = line?;
        let bag_size = line.chars().count();
        let mut half1: HashSet<u8> = line
            .chars()
            .take(bag_size / 2)
            .map(|c| c.priority())
            .collect();
        let mut half2: HashSet<u8> = line
            .chars()
            .skip(bag_size / 2)
            .map(|c| c.priority())
            .collect();
        let same = half1.intersection(&half2).next().unwrap();
        sum += *same as u64;
    }
    println!("{}", sum);
    Ok(())
}

trait Priority {
    fn priority(&self) -> u8;
}

impl Priority for char {
    fn priority(&self) -> u8 {
        match *self {
            'a'..='z' => *self as u8 - 96,
            'A'..='Z' => *self as u8 - 64 + 26,
            _ => 0,
        }
    }
}
