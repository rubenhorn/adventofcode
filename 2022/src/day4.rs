#[macro_use]
extern crate scan_fmt;

fn main() {
    let input = include_str!("../input/day4.txt").trim();
    let ranges_pairs = input
        .lines()
        .map(|s| {
            let (start_1, end_1, start_2, end_2) =
                scan_fmt!(s, "{}-{},{}-{}", u32, u32, u32, u32).unwrap();
            ((start_1, end_1), (start_2, end_2))
        })
        .collect::<Vec<_>>();
    let is_sub_range = |a: (u32, u32), b: (u32, u32)| -> bool { a.0 >= b.0 && a.1 <= b.1 };
    let fully_contained = ranges_pairs
        .iter()
        .filter(|p| is_sub_range(p.0, p.1) || is_sub_range(p.1, p.0))
        .count();
    println!("Fully contained ranges in pairs: {}", fully_contained);
    let do_overlap = |a: (u32, u32), b: (u32, u32)| -> bool {
        (a.0 <= b.0 && a.1 >= b.0)
            || (a.0 >= b.0 && a.1 <= b.1)
            || (b.0 <= a.0 && b.1 >= a.0)
            || (b.0 >= a.0 && b.1 <= a.1)
    };
    let overlapping = ranges_pairs.iter().filter(|p| do_overlap(p.0, p.1)).count();
    println!("Overlapping ranges in pairs: {}", overlapping);
}
