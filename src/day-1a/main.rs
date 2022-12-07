use std::cmp::max;
use std::io;

fn main() -> anyhow::Result<()> {
    let mut largest_sum = 0;
    let mut current_sum = 0;
    for l in io::stdin().lines() {
        let l = l?;
        if l.trim().is_empty() {
            largest_sum = max(largest_sum, current_sum);
            current_sum = 0;
        } else {
            current_sum += l.parse::<u64>()?;
        }
    }
    println!("{}", largest_sum);
    Ok(())
}
