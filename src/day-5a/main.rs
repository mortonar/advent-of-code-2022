use std::io;

fn main() -> anyhow::Result<()> {
    let mut lines = io::stdin().lines();
    let mut stacks = vec![];
    loop {
        if let Some(Ok(line)) = lines.next() {
            if line.trim().is_empty() {
                break;
            }
            stacks.push(line);
        }
    }
    let mut stacks = build_stacks(&stacks).unwrap();

    print_stacks(&stacks);
    for command in lines {
        dbg!(&command);
        run_command(&(command?), &mut stacks);
        print_stacks(&stacks);
    }
    println!("{}", get_top_crates(&stacks));
    Ok(())
}

fn build_stacks(stacks_raw: &Vec<String>) -> Result<Vec<Vec<char>>, ()> {
    let last_row = &stacks_raw[stacks_raw.len() - 1];
    // Vec<(column index, stack index)>
    let stack_column_map: Vec<_> = last_row
        .chars()
        .enumerate()
        .filter(|(i, c)| c.is_ascii_digit())
        .map(|(i, c)| (i, c.to_digit(10).unwrap() as usize))
        .collect();
    let num_stacks = stack_column_map.last().ok_or(())?.1;
    let mut stacks = vec![vec![]; num_stacks];
    for i in (0..stacks_raw.len() - 1).rev() {
        let row_chars: Vec<_> = (&stacks_raw[i]).chars().collect();
        for (i, c) in stack_column_map.iter() {
            if let Some(crate_char) = row_chars.get(*i) {
                if !crate_char.is_ascii_whitespace() {
                    stacks[*c - 1].push(*crate_char);
                }
            }
        }
    }
    Ok(stacks)
}

fn run_command(command: &str, stacks: &mut Vec<Vec<char>>) -> Result<(), ()> {
    let mut command: Vec<usize> = command
        .split_ascii_whitespace()
        .filter_map(|t| t.parse::<usize>().ok())
        .collect();
    let to_idx = command.pop().ok_or(())? - 1;
    let from_idx = command.pop().ok_or(())? - 1;
    let amount = command.pop().ok_or(())?;

    for i in 0..amount {
        if let Some(cr8) = stacks[from_idx].pop() {
            stacks[to_idx].push(cr8);
        }
    }

    Ok(())
}

fn get_top_crates(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack.last().unwrap())
        .collect()
}

fn print_stacks(stacks: &Vec<Vec<char>>) {
    for (i, stack) in stacks.iter().enumerate() {
        print!("{}({}): ", i + 1, stack.len());
        for cr8 in stack.iter() {
            print!("{}", cr8);
        }
        println!();
    }
}
