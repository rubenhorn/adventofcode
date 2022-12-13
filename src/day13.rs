use std::cmp::Ord;
use std::fmt::Debug;

use itertools::Itertools;

#[derive(PartialEq, Eq)]
enum ListItem {
    List(Vec<ListItem>),
    Int(u32),
}

impl Debug for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListItem::List(l) => write!(f, "{}", format!("{:?}", l).replace(' ', "")),
            ListItem::Int(i) => write!(f, "{}", i),
        }
    }
}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (ListItem::Int(left), ListItem::Int(right)) => left.cmp(right),
            (ListItem::List(left), ListItem::List(right)) => left.cmp(right),
            (ListItem::List(left), ListItem::Int(right)) => left.cmp(&vec![ListItem::Int(*right)]),
            (ListItem::Int(left), ListItem::List(right)) => vec![ListItem::Int(*left)].cmp(right),
        }
    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(s: &str, start_at: usize) -> (Vec<ListItem>, usize) {
    let mut char_idx = start_at;
    let mut res = Vec::new();
    let chars = s.chars().collect_vec();
    let mut next_int_str = String::new();
    assert_eq!(chars[0], '[');
    char_idx += 1;
    while char_idx < s.len() {
        match chars[char_idx] {
            ',' => {
                if !next_int_str.is_empty() {
                    res.push(ListItem::Int(next_int_str.parse().unwrap()));
                    next_int_str.clear();
                }
            }
            '[' => {
                let list: Vec<ListItem>;
                (list, char_idx) = parse(s, char_idx);
                res.push(ListItem::List(list));
            }
            ']' => {
                if !next_int_str.is_empty() {
                    res.push(ListItem::Int(next_int_str.parse().unwrap()));
                }
                break;
            }
            c => next_int_str.push(c),
        }
        char_idx += 1;
    }
    if start_at == 0 {
        assert_eq!(s, format!("{:?}", res).replace(' ', ""));
    }
    (res, char_idx)
}

fn main() {
    let input = include_str!("../input/day13.txt").trim();
    let pairs = input
        .split("\n\n")
        .map(|s| s.lines().map(|s| parse(&s, 0).0).collect_tuple().unwrap())
        .collect_vec();
    println!(
        "Sum of correctly ordered indices: {}",
        pairs
            .iter()
            .enumerate()
            .filter_map(|(i, (left, right))| {
                if left < right {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .sum::<usize>()
    );
    // Part 2:
    macro_rules! divider {
        ($i:expr) => {
            &vec![ListItem::List(vec![ListItem::Int($i)])]
        };
    }
    let dividers = [divider!(2), divider!(6)];
    println!(
        "Decoder key: {}",
        pairs
            .iter()
            .flat_map(|p| vec![&p.0, &p.1])
            .chain(dividers)
            .sorted()
            .enumerate()
            .filter_map(|(i, p)| {
                if dividers.contains(&p) {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .reduce(|a, b| a * b)
            .unwrap()
    );
}
