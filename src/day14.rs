use std::cmp::{max, min};

use itertools::Itertools;

type Point = (usize, usize);

fn main() {
    let input = include_str!("../input/day14.txt").trim();
    let mut rock_formations: Vec<Vec<Point>> = input
        .lines()
        .map(|s| {
            s.split(" -> ")
                .map(|s| {
                    s.split(',')
                        .map(|s| s.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();
    let find_vertex_by = |reducer: &dyn Fn(Point, Point) -> Point| {
        rock_formations
            .iter()
            .flatten()
            .map(|v| *v)
            .reduce(reducer)
            .unwrap()
    };
    let mut sand_spawn: Point = (500, 0);
    let x_min = min(
        sand_spawn.0,
        find_vertex_by(&|a, b| if a.0 < b.0 { a } else { b }).0,
    );
    let y_min = min(
        sand_spawn.1,
        find_vertex_by(&|a, b| if a.1 < b.1 { a } else { b }).1,
    );
    let x_max = max(
        sand_spawn.0,
        find_vertex_by(&|a, b| if a.0 > b.0 { a } else { b }).0,
    ) - x_min;
    let y_max = max(
        sand_spawn.1,
        find_vertex_by(&|a, b| if a.1 > b.1 { a } else { b }).1,
    ) - y_min;
    rock_formations.iter_mut().for_each(|f| {
        f.iter_mut().for_each(|v| {
            v.0 -= x_min;
            v.1 -= y_min;
        });
    });
    sand_spawn = (sand_spawn.0 - x_min + 1, sand_spawn.1 - y_min);
    let mut world = vec![vec!['.'; x_max + 1 + 2]; y_max + 1];
    world[sand_spawn.1][sand_spawn.0] = '+';
    rock_formations.iter().for_each(|f| {
        for (from, to) in f.iter().tuple_windows() {
            if from.0 != to.0 {
                for x in min(from.0, to.0)..max(from.0, to.0) + 1 {
                    world[from.1][x + 1] = '#';
                }
            } else {
                for y in min(from.1, to.1)..max(from.1, to.1) + 1 {
                    world[y][from.0 + 1] = '#';
                }
            }
        }
    });
    fn print_world(world: &Vec<Vec<char>>) {
        for row in world {
            println!("{}", row.iter().join(""));
        }
    }
    let drop_sand = |world: &mut Vec<Vec<char>>| {
        let sand_spawn = (world[0].iter().position(|c| *c == '+').unwrap(), 0);
        let mut sand = sand_spawn.clone();
        let y_max = world.len() - 1;
        loop {
            // world[sand.1][sand.0] = '~';
            let sand_next = *[(0, 1), (-1, 1), (1, 1), (0, 0)]
                .iter()
                .filter_map(|offset| {
                    let sand = (
                        (sand.0 as i32 + offset.0) as usize,
                        (sand.1 as i32 + offset.1) as usize,
                    );
                    if ['.', '~', '+'].contains(&world[sand.1][sand.0]) {
                        Some(sand)
                    } else {
                        None
                    }
                })
                .collect_vec()
                .first()
                .unwrap();
            if sand_next == sand {
                break;
            }
            sand = sand_next;
            if sand.1 >= y_max {
                return false;
            }
        }
        world[sand.1][sand.0] = 'o';
        sand != sand_spawn
    };
    while drop_sand(&mut world) {
        // print_world(&world);
        // println!("{}", world[0].iter().map(|_| '=').join(""))
    }
    println!(
        "{} grains of sand",
        world.iter().flatten().filter(|c| **c == 'o').count()
    );
    println!("\n");
    // Part 2:
    world.iter_mut().for_each(|r| {
        r.iter_mut().for_each(|c| {
            // Clear sand
            if *c == 'o' {
                *c = '.';
            }
        });
    });
    while world.len() * world.len() > world[0].len() {
        world.iter_mut().for_each(|r| {
            r.insert(0, '.');
            r.push('.');
        });
    }
    world.push(vec!['.'; world[0].len()]);
    world.push(vec!['#'; world[0].len()]);
    while drop_sand(&mut world) {
        // print_world(&world);
        // println!("{}", world[0].iter().map(|_| '=').join(""))
    }
    print_world(&world);
    println!(
        "{} grains of sand",
        world.iter().flatten().filter(|c| **c == 'o').count()
    );
}
