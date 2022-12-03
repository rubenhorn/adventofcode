fn main() {
    let input = include_str!("../input/day3.txt").trim();

    let get_priority = |c| {
        if c < 'a' {
            c as u32 - 'A' as u32 + 1 + 26
        } else {
            c as u32 - 'a' as u32 + 1
        }
    };
    // Part 1
    let duplicates = input.lines().map(|s| {
        let (first_compartment, second_compartment) = s.split_at(s.len() / 2);
        first_compartment
            .chars()
            .filter(|c| second_compartment.contains(*c))
            .collect::<Vec<_>>()[0]
    });
    let priorities_1 = duplicates.map(get_priority);
    println!(
        "Sum of priorities for part 1: {}",
        priorities_1.sum::<u32>()
    );
    // Part 2
    let rucksacks = input.lines().collect::<Vec<_>>();
    let priorities_2 = (0..rucksacks.len())
        .step_by(3)
        .map(|i| {
            rucksacks[i]
                .chars()
                .filter(|c| rucksacks[i + 1].contains(*c) && rucksacks[i + 2].contains(*c))
                .collect::<Vec<_>>()[0]
        })
        .map(get_priority);
    println!(
        "Sum of priorities for part 1: {}",
        priorities_2.sum::<u32>()
    );
}
