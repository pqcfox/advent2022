use itertools::Itertools;

pub fn run(input: &str) {
    let mut part_total: u32 = 0;
    let mut badge_total: u32 = 0;

    // work in chunks of three lines, representing groups
    for group in &input.lines().chunks(3) {
        let group_lines: Vec<_> = group.collect();

        // figure out the badge and add its priority
        for c in group_lines[0].chars() {
            if group_lines[1].contains(c) && group_lines[2].contains(c) {
                badge_total += compute_priority(c) as u32;
                break;
            }
        }

        // for each line in the group, find the repeated item and add its priority
        for line in group_lines {
            let divide = line.len() / 2;
            let first_part = &line[..divide];
            let second_part = &line[divide..];
            for c in first_part.chars() {
                if second_part.contains(c) {
                    part_total += compute_priority(c) as u32;
                    break;
                };
            }
        }
    }

    println!("Total rucksack partition priority: {}", part_total);
    println!("Total badge priority: {}", badge_total);
}

fn compute_priority(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - 0x60,
        'A'..='Z' => c as u8 - 0x40 + 26,
        _ => panic!("Invalid rucksack content"),
    }
}
