use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input/day9.txt").trim();
    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut visited = HashSet::<(i32, i32)>::new();
    visited.insert(t);
    let steps = input
        .lines()
        .flat_map(|s| {
            let (direction, steps) = s.split(' ').collect_tuple().unwrap();
            (0..steps.parse::<usize>().unwrap()).map(|_| direction.to_owned())
        })
        .collect_vec();
    fn move_point(point: &mut (i32, i32), direction: &str) {
        match direction {
            "L" => point.0 -= 1,
            "R" => point.0 += 1,
            "U" => point.1 -= 1,
            "D" => point.1 += 1,
            _ => panic!("Invalid input!"),
        };
    }
    fn follow(p1: (i32, i32), p2: &mut (i32, i32)) {
        if (p1.0 - p2.0).abs() > 1 || (p1.1 - p2.1).abs() > 1 {
            if p1.0 != p2.0 {
                p2.0 += (p1.0 - p2.0).signum();
            }
            if p1.1 != p2.1 {
                p2.1 += (p1.1 - p2.1).signum();
            }
        }
    }
    steps.iter().for_each(|direction| {
        move_point(&mut h, direction.as_str());
        follow(h, &mut t);
        visited.insert(t);
    });
    println!("Visited {} positions (H-T)", visited.len());
    visited.clear();
    let mut rope = (0..10).map(|_| (0, 0)).collect_vec();
    visited.insert(rope[rope.len() - 1]);
    steps.iter().for_each(|direction| {
        move_point(&mut rope[0], direction.as_str());
        (1..rope.len()).for_each(|i| {
            follow(rope[i-1], &mut rope[i]);
        });
        visited.insert(rope[rope.len() - 1]);
    });
    println!("Visited {} positions (H-2-3-...-T)", visited.len());
}
