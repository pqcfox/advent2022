use std::iter::zip;

#[derive(Debug, Clone, Copy)]
struct Command {
    count: usize,
    from_idx: usize,
    to_idx: usize,
}

pub fn run(input: &str) {
    let mut lines = input.lines();
    let state_lines: Vec<&str> = lines.by_ref().take_while(|l| !l.is_empty()).collect();
    let mut original_stacks = parse_initial_state(&state_lines);
    let mut updated_stacks = original_stacks.clone();

    for line in lines {
        let cmd = parse_command_line(line);

        process_command(cmd, &mut original_stacks, false);
        process_command(cmd, &mut updated_stacks, true);
    }

    let top_str = collect_string_from_stacks(&original_stacks);
    let updated_top_str = collect_string_from_stacks(&updated_stacks);

    println!(
        "String formed by tops of stacks when moving one at a time: {}",
        top_str
    );
    println!(
        "String formed by tops of stacks when moving many at a time: {}",
        updated_top_str
    );
}

fn parse_initial_state(state_lines: &[&str]) -> Vec<Vec<char>> {
    let num_stacks = state_lines
        .last()
        .expect("No state found in input")
        .split_whitespace()
        .last()
        .expect("No column numbers found below state")
        .parse()
        .expect("Failed to parse column numbers");

    let mut stacks = vec![Vec::<char>::new(); num_stacks];
    let (_, stack_lines) = state_lines.split_last().unwrap();

    for stack_line in stack_lines.iter().rev() {
        let crate_chars = (1..=(1 + 4 * (num_stacks - 1))).step_by(4).map(|i| {
            stack_line
                .chars()
                .nth(i.into())
                .expect("State line too short")
        });

        for (c, stack) in zip(crate_chars, stacks.iter_mut()) {
            if c != ' ' {
                stack.push(c);
            }
        }
    }

    stacks
}

fn parse_command_line(line: &str) -> Command {
    let command_parts: Vec<&str> = line.split_whitespace().collect();
    let count: usize = command_parts[1].parse().expect("Failed to parse count");
    let from_idx: usize = command_parts[3]
        .parse()
        .expect("Failed to parse from index");
    let to_idx: usize = command_parts[5].parse().expect("Failed to parse to index");

    Command {
        count,
        from_idx: from_idx - 1,
        to_idx: to_idx - 1,
    }
}

fn collect_string_from_stacks(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .map(|stack| stack.last().expect("Empty stack at end"))
        .collect()
}

fn process_command(cmd: Command, stacks: &mut Vec<Vec<char>>, updated: bool) {
    let src: &mut Vec<char> = &mut stacks[cmd.from_idx];
    let mut moved_chars: Vec<char> = src.drain((src.len() - cmd.count)..).collect();
    let dest: &mut Vec<char> = &mut stacks[cmd.to_idx];
    if !updated {
        moved_chars.reverse();
    }
    dest.extend(moved_chars.iter());
}
