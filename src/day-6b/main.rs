use std::io;

fn main() -> anyhow::Result<()> {
    let data_stream: Vec<_> = io::stdin().lines().next().unwrap()?.chars().collect();
    let (i, start_seq) = data_stream
        .windows(14)
        .enumerate()
        .find(|(i, seq)| all_uniq(seq))
        .unwrap();
    println!("{}", i + 14);
    Ok(())
}

fn all_uniq(seq: &[char]) -> bool {
    for i in 0..(seq.len() - 1) {
        for j in (i + 1)..(seq.len()) {
            if seq[i] == seq[j] {
                return false;
            }
        }
    }
    true
}
