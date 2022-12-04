pub fn run(input: &str) {
    let mut top_calories: [u32; 3] = [0; 3];
    let mut total_calories: u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            let lowest_top = top_calories.iter_mut().min().unwrap();
            if total_calories > *lowest_top {
                *lowest_top = total_calories;
            }
            total_calories = 0;
        } else {
            let calories: u32 = line.parse().expect("Failed to parse calories");
            total_calories += calories;
        }
    }

    let max_calories: u32 = *top_calories.iter().max().expect("No calories provided");
    let sum_calories: u32 = top_calories.iter().sum();

    println!("Maximum calorie count: {max_calories}");
    println!("Sum of top three calorie counts: {sum_calories}");
}
