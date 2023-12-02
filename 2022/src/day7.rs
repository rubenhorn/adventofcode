use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("../input/day7.txt");
    let mut cwd = Vec::<String>::new();
    let mut folders = HashSet::<String>::new();
    let mut files = HashMap::<String, usize>::new();
    let mut is_listing = false;
    input.lines().for_each(|s| {
        let cols = s.split(' ').collect_vec();
        if cols[0] == "$" {
            is_listing = false;
        }
        if cols[0..=1] == ["$", "cd"] {
            match cols[2] {
                "/" => cwd.clear(),
                ".." => {
                    cwd.pop();
                }
                folder => {
                    cwd.push(folder.to_string());
                    folders.insert(cwd.join("/"));
                }
            };
        } else if cols[0..=1] == ["$", "ls"] {
            is_listing = true;
        } else if is_listing && cols[0] != "dir" {
            files.insert(
                format!("{}/{}", cwd.join("/").to_string(), cols[1]),
                cols[0].parse().unwrap(),
            );
        }
    });

    let mut folder_sizes = folders
        .iter()
        .map(|folder| {
            (
                format!("/{}", folder),
                files
                    .iter()
                    .filter(|(f, _)| f.starts_with(&format!("{}/", folder)))
                    .map(|(_, s)| s)
                    .sum::<usize>(),
            )
        })
        .collect_vec();
    folder_sizes.insert(0, ("/".to_string(), files.iter().map(|(_, s)| s).sum()));

    println!(
        "Size of folders up to 100000: {}",
        folder_sizes
            .iter()
            .filter(|(_, s)| *s <= 100000)
            .map(|(_, s)| s)
            .sum::<usize>()
    );

    let required_space = 30000000;
    let available_space = 70000000 - folder_sizes[0].1;
    println!(
        "Size of folder that should be deleted: {}",
        folder_sizes
            .iter()
            .filter(|(_, s)| required_space <= available_space + s)
            .map(|(_, s)| s)
            .min()
            .unwrap()
    );
}
