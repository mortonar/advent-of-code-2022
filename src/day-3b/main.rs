use std::collections::HashSet;
use std::io;

fn main() -> anyhow::Result<()> {
    let mut sum: u64 = 0;
    let mut lines = io::stdin().lines().peekable();
    while lines.peek().is_some() {
        let group: Vec<HashSet<u8>> = lines
            .by_ref()
            .take(3)
            .map(|elf| elf.unwrap().chars().map(|c| c.priority()).collect())
            .collect();
        let same = group[0]
            .iter()
            .find(|e| group[1].contains(&e) && group[2].contains(&e))
            .unwrap();
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
