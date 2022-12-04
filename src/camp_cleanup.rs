pub fn run(input: &str) {
    let mut nest_count: u32 = 0;
    let mut overlap_count: u32 = 0;

    for line in input.lines() {
        let ranges: Vec<(u8, u8)> = line.split(",").map(parse_range).collect();

        if range_inside(ranges[0], ranges[1]) || range_inside(ranges[1], ranges[0]) {
            nest_count += 1;
            overlap_count += 1;
        } else if range_has_endpoint_in(ranges[0], ranges[1]) {
            overlap_count += 1;
        }
    }

    println!("Number of nested assignments: {}", nest_count);
    println!("Number of overlapped assignments: {}", overlap_count);
}

fn parse_range(input: &str) -> (u8, u8) {
    let parts: Vec<&str> = input.split("-").collect();
    let start: u8 = parts[0].parse().expect("Failed to parse start of range");
    let end: u8 = parts[1].parse().expect("Failed to parse end of range");
    (start, end)
}

fn range_inside(range: (u8, u8), other: (u8, u8)) -> bool {
    range.0 >= other.0 && range.1 <= other.1
}

fn range_has_endpoint_in(pair: (u8, u8), other: (u8, u8)) -> bool {
    (other.0 <= pair.0 && pair.0 <= other.1) || (other.0 <= pair.1 && pair.1 <= other.1)
}
