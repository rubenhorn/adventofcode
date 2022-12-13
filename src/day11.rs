use std::fmt::Display;

use itertools::Itertools;
use scan_fmt::scan_fmt;

#[derive(Debug, Clone)]
struct Monkey {
    number: usize,
    items: Vec<i64>,
    operator: char,
    operands: (Value, Value),
    test_division_by: i64,
    monkey_true: usize,
    monkey_false: usize,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Monkey {}: {}",
            self.number,
            self.items.iter().join(", ")
        )
    }
}

#[derive(Debug, Clone)]
enum Value {
    Old,
    Literal(u32),
}

fn monkey_business(
    monkeys_ref: &Vec<Monkey>,
    rounds: usize,
    experiences_relief: bool,
    print_debug: bool,
) {
    let mut monkeys = monkeys_ref.clone();
    let mut inspection_counts = vec![0usize; monkeys.len()];
    let monkey_modulo = monkeys
        .iter()
        .map(|monkey| monkey.test_division_by)
        .reduce(|a, b| a * b)
        .unwrap();

    macro_rules! debugln {
        ($s:expr) => {
            if print_debug {
                println!("{}", $s)
            }
        };
    }

    let mut monkey_business_round = || {
        for i in 0..monkeys.len() {
            let mut_monkeys = &mut monkeys;
            debugln!(format!("Monkey {}:", mut_monkeys[i].number));
            let items = mut_monkeys[i].items.clone();
            inspection_counts[i] += items.len();
            mut_monkeys[i].items.clear();
            for item in items {
                debugln!(format!(
                    "   Monkey inspects an item with a worry level of {}.",
                    item
                ));
                let operand_1 = match mut_monkeys[i].operands.0 {
                    Value::Old => item,
                    Value::Literal(i) => i64::from(i),
                };
                let operand_2 = match mut_monkeys[i].operands.1 {
                    Value::Old => item,
                    Value::Literal(i) => i64::from(i),
                };
                let mut updated_item = match mut_monkeys[i].operator {
                    '+' => operand_1 + operand_2,
                    '-' => operand_1 - operand_2,
                    '*' => operand_1 * operand_2,
                    '/' => operand_1 / operand_2,
                    _ => panic!("Invalid operator!"),
                };
                debugln!(format!("      Worry level is updated to {}", updated_item));
                if experiences_relief {
                    updated_item /= 3;
                    debugln!(format!(
                        "      Monkey gets bored with item. Worry level is divided by 3 to {}.",
                        updated_item
                    ));
                }
                let next_monkey = if updated_item % mut_monkeys[i].test_division_by == 0 {
                    debugln!(format!(
                        "      Current worry level is divisible by {}.",
                        mut_monkeys[i].test_division_by
                    ));
                    mut_monkeys[i].monkey_true
                } else {
                    debugln!(format!(
                        "      Current worry level is not divisible by {}.",
                        mut_monkeys[i].test_division_by
                    ));
                    mut_monkeys[i].monkey_false
                };
                debugln!(format!(
                    "      Item with worry level 500 is thrown to monkey {}.",
                    next_monkey
                ));
                mut_monkeys[next_monkey]
                    .items
                    .push(updated_item % monkey_modulo);
            }
        }
        for i in 0..monkeys.len() {
            debugln!(format!("{}", monkeys[i]));
        }
        debugln!(format!(""))
    };

    for _ in 0..rounds {
        monkey_business_round();
    }
    for i in 0..monkeys.len() {
        println!(
            "Monkey {} inspected items {} times.",
            i, inspection_counts[i]
        );
    }
    inspection_counts.sort();
    inspection_counts.reverse();
    println!(
        "The level of monkey business is {}",
        inspection_counts[0] * inspection_counts[1]
    );
}

fn main() {
    let input = include_str!("../input/day11.txt").trim();
    let monkey_inputs = input.split("\n\n").collect_vec();
    let parse_monkey = |s: &str| -> Option<Monkey> {
        let lines = s.lines().map(|s| s.trim()).collect_vec();
        let number = scan_fmt!(lines[0], "Monkey {}:", usize).unwrap();
        let items = lines[1]
            .split(':')
            .nth(1)?
            .split(',')
            .map(|s| String::from(s.trim()).parse().unwrap())
            .collect_vec();
        let operator = lines[2].split("old").nth(1)?.trim().chars().nth(0)?;
        let operands = lines[2]
            .split('=')
            .nth(1)?
            .split(operator)
            .map(|s| {
                if s.trim() == "old" {
                    Value::Old
                } else {
                    Value::Literal(s.trim().parse().unwrap())
                }
            })
            .collect_tuple::<(Value, Value)>()?;
        let test_division_by = lines[3].split("by ").nth(1)?.parse().unwrap();
        let monkey_true = lines[4].split("monkey ").nth(1)?.parse().unwrap();
        let monkey_false = lines[5].split("monkey ").nth(1)?.parse().unwrap();
        Some(Monkey {
            number,
            items,
            operator,
            operands,
            test_division_by,
            monkey_true,
            monkey_false,
        })
    };
    let monkeys = monkey_inputs
        .iter()
        .map(|s| parse_monkey(s).unwrap())
        .collect_vec();
    println!("Part 1:");
    monkey_business(&monkeys, 20, true, true);
    println!("\nPart 2:");
    monkey_business(&monkeys, 10000, false, false);
}
