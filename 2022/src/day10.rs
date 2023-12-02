use itertools::Itertools;

fn main() {
    let input = include_str!("../input/day10.txt").trim();
    let mut adds = input
        .lines()
        .flat_map(|s| {
            if s == "noop" {
                vec![0] // noop is the same as adding 0
            } else {
                vec![0, s.split(' ').nth(1).unwrap().parse().unwrap()]
            }
        })
        .collect_vec();
    adds.insert(0, 0); // Starts delay all writes by 1 cycle (does not affect noop)
    let mut x_t = vec![1];
    adds.iter().for_each(|v| {
        x_t.push(v + x_t.last().unwrap());
    });
    let signal_strengths = [20, 60, 100, 140, 180, 220]
        .map(|i| i as i32 * x_t[i])
        .iter()
        .sum::<i32>();
    println!("Signal strenght: {:?}", signal_strengths);

    let empty_row = vec!['.'; 40];
    for i in 1..x_t.len() {
        let mut row = empty_row.clone();
        if (i - 1) % row.len() == 0 {
            println!();
        }
        let sprite_position = x_t[i];
        (0..3).for_each(|i| {
            let pixel = sprite_position + i;
            if pixel >= 0 && pixel < row.len().try_into().unwrap() {
                row[pixel as usize] = '#';
            }
        });
        print!("{}", row[i % row.len()]);
    }
    // ^-- Not quite right but good enough :P
}
