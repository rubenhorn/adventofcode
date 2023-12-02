fn main() {
    let input = include_str!("../input/day1.txt").trim();
    let mut elves = input
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
        })
        .collect::<Vec<_>>();
    elves.sort_by_key(|e| e.1);
    elves.reverse();
    println!(
        "Elve #{} has the most ({}) calories",
        elves[0].0, elves[0].1
    );
    println!(
        "Top 3 calories combined: {}",
        elves[0].1 + elves[1].1 + elves[2].1
    );
}
