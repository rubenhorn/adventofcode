fn main() {
    let input = include_str!("../input/day1.txt").trim();
    let elves = input
        .split("\n\n")
        .enumerate()
        // Compute total calories
        .map(|(elve_number, calories)| {
            let total_calories = calories
                .split('\n')
                .map(|s| s.parse::<u32>().unwrap())
                .reduce(|sum, item| sum + item)
                .unwrap();
            (elve_number, total_calories)
        });
    let elve_max_calories = elves
        .reduce(|max, item| if max.1 > item.1 { max } else { item })
        .unwrap();
    println!(
        "Elve #{} has the most ({}) calories",
        elve_max_calories.0, elve_max_calories.1
    );
}
