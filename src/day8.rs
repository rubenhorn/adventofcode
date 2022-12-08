use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input/day8.txt").trim();
    let forrest = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| String::from(c).parse::<u8>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let mut visible = HashSet::<(usize, usize)>::new();
    // Horizontal check
    for i in 0..forrest.len() {
        visible.insert((i, 0));
        let mut max = forrest[i][0];
        for j in 1..forrest[i].len() {
            if forrest[i][j] > max {
                visible.insert((i, j));
                max = forrest[i][j];
            }
        }
        visible.insert((i, forrest[0].len() - 1));
        max = forrest[i][forrest[0].len() - 1];
        for j in (0..forrest[i].len() - 1).rev() {
            if forrest[i][j] > max {
                visible.insert((i, j));
                max = forrest[i][j];
            }
        }
    }
    // Vertical check
    for j in 0..forrest[0].len() {
        visible.insert((0, j));
        let mut max = forrest[0][j];
        for i in 1..forrest.len() {
            if forrest[i][j] > max {
                visible.insert((i, j));
                max = forrest[i][j];
            }
        }
        visible.insert((forrest[0].len() - 1, j));
        max = forrest[forrest[0].len() - 1][j];
        for i in (0..forrest.len() - 1).rev() {
            if forrest[i][j] > max {
                visible.insert((i, j));
                max = forrest[i][j];
            }
        }
    }
    println!("\nVisible trees: {:?}", visible.iter().count());

    let compute_visibility2 = |i: usize, j: usize| {
        fn count_visible2(trees: &Vec<u8>) -> usize {
            // First tree is only threshold
            let mut visible = 0;
            for i in 1..trees.len() {
                visible += 1;
                if trees[i] >= trees[0] {
                    break;
                }
            }
            visible
        }
        let trees_up = (0..=i).rev().map(|i2| forrest[i2][j]).collect_vec();
        let visible_up = count_visible2(&trees_up);
        let trees_down = (i..forrest.len()).map(|i2| forrest[i2][j]).collect_vec();
        let visible_down = count_visible2(&trees_down);
        let trees_left = (0..=j).rev().map(|j2| forrest[i][j2]).collect_vec();
        let visible_left = count_visible2(&trees_left);
        let trees_right = (j..forrest[0].len()).map(|j2| forrest[i][j2]).collect_vec();
        let visible_right = count_visible2(&trees_right);
        let score = visible_up * visible_down * visible_left * visible_right;
        score
    };

    println!(
        "Best tree house visibility score: {}",
        (0..forrest.len())
            .flat_map(|i| (0..forrest[0].len()).map(move |j| compute_visibility2(i, j)))
            .max()
            .unwrap()
    );
}
