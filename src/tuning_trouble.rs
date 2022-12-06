pub fn run(input: &str) {
    let chars: Vec<char> = input.trim().chars().collect();
    let packet_index = find_marker(&chars, 4);
    let message_index = find_marker(&chars, 14);

    println!("Start-of-packet marker detected at index {}", packet_index);
    println!(
        "Start-of-message marker detected at index {}",
        message_index
    );
}

fn find_marker(chars: &[char], marker_len: usize) -> usize {
    let mut index: usize = marker_len;
    for window in chars.windows(marker_len) {
        let mut sorted = Vec::from(window);
        sorted.sort();

        // yes, I could have used itertools, but I thought this was cute
        if sorted.windows(2).map(|w| w[0] != w[1]).all(|b| b) {
            break;
        }
        index += 1;
    }

    index
}
