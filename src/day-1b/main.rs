use std::io;

fn main() -> anyhow::Result<()> {
    let mut largest_sums = vec![0; 3];
    let mut current_sum = 0;
    for l in io::stdin().lines() {
        let l = l?;
        if l.trim().is_empty() {
            largest_sums.push(current_sum);
            largest_sums.sort();
            largest_sums.remove(0);
            current_sum = 0;
        } else {
            current_sum += l.parse::<u64>()?;
        }
    }
    println!("{}", largest_sums.iter().sum::<u64>());
    Ok(())
}
