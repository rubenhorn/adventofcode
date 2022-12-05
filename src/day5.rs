use itertools::Itertools;

#[macro_use]
extern crate scan_fmt;

fn main() {
    let input = include_str!("../input/day5.txt");
    let (start_state, moves) = input.split("\n\n").into_iter().collect_tuple().unwrap();

    let create_state = |s: &str| {
        let mut state = s.lines()
            .last()
            .unwrap()
            .split(char::is_whitespace)
            .filter(|s| s.len() > 0)
            .map(|_| Vec::<char>::new())
            .collect_vec();
        start_state.lines().rev().skip(1).for_each(|s| {
            let chars = s.chars().collect_vec();
            for i in 0..state.len() {
                let c = chars[1 + i + i * 3];
                if c.is_alphabetic() && c.is_uppercase() {
                    state[i].push(c);
                }
            }
        });
        state
    };

    let mut state = create_state(start_state);
    moves
        .trim()
        .lines()
        .map(|s| {
            let (repeat, from, to) =
                scan_fmt!(s, "move {} from {} to {}", usize, usize, usize).unwrap();
            (0..repeat).map(|_| (from, to)).collect_vec()
        })
        .flatten()
        .for_each(|m| {
            let c = state[m.0 - 1].pop().unwrap();
            state[m.1 - 1].push(c);
        });
    println!(
        "Top items (CrateMover 9000): {}",
        state.iter().map(|v| v.last().unwrap()).collect::<String>()
    );

    state = create_state(start_state);
    moves
        .trim()
        .lines()
        .map(|s| scan_fmt!(s, "move {} from {} to {}", usize, usize, usize).unwrap())
        .for_each(|m| {
            let mut stack = Vec::<char>::new();
            for _ in 0..m.0 {
                stack.push(state[m.1 - 1].pop().unwrap());
            }
            for _ in 0..m.0 {
                state[m.2 - 1].push(stack.pop().unwrap());
            }
        });
    println!(
        "Top items (CrateMover 9001): {}",
        state.iter().map(|v| v.last().unwrap()).collect::<String>()
    );
}
