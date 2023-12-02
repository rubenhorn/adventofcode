fn main() {
    let input = include_str!("../input/day2.txt").trim();
    let score_part_1 = input
        .lines()
        .map(|s| {
            let opponent_move = s.chars().nth(0)? as u32 - 'A' as u32;
            let own_move = s.chars().nth(2)? as u32 - 'X' as u32;
            let score = if (opponent_move + 1) % 3 == own_move {
                6
            } else if opponent_move == own_move {
                3
            } else {
                0
            };
            Some(score + own_move + 1)
        })
        .reduce(|sum, item| Some(sum? + item?))
        .unwrap()
        .unwrap();
    println!("Final score part 1: {}", score_part_1);
    let score_part_2 = input
        .lines()
        .map(|s| {
            let opponent_move = s.chars().nth(0)? as u32 - 'A' as u32;
            let outcome = s.chars().nth(2)?;
            Some(if outcome == 'X' {
                0 + (if opponent_move == 0 {
                    2
                } else {
                    opponent_move - 1
                }) + 1
            } else if outcome == 'Y' {
                3 + opponent_move + 1
            } else {
                6 + (opponent_move + 1) % 3 + 1
            })
        })
        .reduce(|sum, item| Some(sum? + item?))
        .unwrap()
        .unwrap();
    println!("Final score part 2: {}", score_part_2);
}
