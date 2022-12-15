use std::io;
use std::num::ParseIntError;

fn main() -> anyhow::Result<()> {
    let contain_pairs = io::stdin()
        .lines()
        .map(|l| parse_pair(&l.unwrap()).unwrap())
        .filter(|range_pair| {
            (range_pair[0] <= range_pair[2] && range_pair[1] >= range_pair[3])
                || (range_pair[0] >= range_pair[2] && range_pair[1] <= range_pair[3])
        })
        .count();
    println!("{}", &contain_pairs);
    Ok(())
}

fn parse_pair(line: &str) -> Result<Vec<u8>, ParseIntError> {
    line.split(',')
        .flat_map(|range| range.split('-'))
        .map(|num| num.parse::<u8>())
        .collect()
}
