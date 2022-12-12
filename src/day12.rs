use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Location {
    Start,
    End,
    Height(i32),
}

impl Location {
    fn height(&self) -> i32 {
        match self {
            Location::Start => 0,
            Location::End => ('z' as u8 - 'a' as u8).into(),
            Location::Height(i) => *i,
        }
    }
}

fn main() {
    let input = include_str!("../input/day12.txt").trim();
    let mut coordinates_s = (0, 0);
    let mut coordinates_e = (0, 0);
    let mut locations = input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            s.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        coordinates_s = (i as i32, j as i32);
                        Location::Start
                    } else if c == 'E' {
                        coordinates_e = (i as i32, j as i32);
                        Location::End
                    } else {
                        Location::Height((c as u8 - 'a' as u8).into())
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    let mut steps_from_s = vec![vec![-1; locations[0].len()]; locations.len()];
    fn update_steps_from_s(
        l: &Vec<Vec<Location>>,
        d: &mut Vec<Vec<i32>>,
        c: (i32, i32),
        t: Location,
        backwards: bool,
    ) {
        let l1 = &l[c.0 as usize][c.1 as usize];
        let will_visit = vec![
            (c.0 - 1, c.1),
            (c.0 + 1, c.1),
            (c.0, c.1 - 1),
            (c.0, c.1 + 1),
        ]
        .iter()
        .filter(|c| c.0 >= 0 && c.0 < l.len() as i32 && c.1 >= 0 && c.1 < l[0].len() as i32)
        .filter(|c| {
            let l2 = &l[c.0 as usize][c.1 as usize];
            if backwards {
                l1.height() <= l2.height() + 1
            } else {
                l2.height() <= l1.height() + 1
            }
        })
        .map(|c| *c)
        .collect_vec();
        let steps_from_c = 1 + d[c.0 as usize][c.1 as usize];
        for c2 in will_visit {
            let steps_old = d[c2.0 as usize][c2.1 as usize];
            if steps_old == -1 || steps_old > steps_from_c {
                d[c2.0 as usize][c2.1 as usize] = steps_from_c;
                update_steps_from_s(l, d, c2, t, backwards);
            }
        }
    }
    steps_from_s[coordinates_s.0 as usize][coordinates_s.1 as usize] = 0;
    update_steps_from_s(
        &locations,
        &mut steps_from_s,
        coordinates_s,
        Location::End,
        false,
    );
    let min_steps_to_e = steps_from_s[coordinates_e.0 as usize][coordinates_e.1 as usize];
    println!("S-E in {} steps", min_steps_to_e);
    // Part 2:
    locations[coordinates_s.0 as usize][coordinates_s.1 as usize] = Location::Height(0);
    let mut steps_from_e = vec![vec![-1; locations[0].len()]; locations.len()];
    steps_from_e[coordinates_e.0 as usize][coordinates_e.1 as usize] = 0;
    update_steps_from_s(
        &locations,
        &mut steps_from_e,
        coordinates_e,
        Location::Height(0),
        true,
    );
    let coordinate_list_start = (0..locations.len())
        .flat_map(|i| (0..locations.len()).map(move |j| (i, j)))
        .filter(|c| locations[c.0][c.1].height() == 0)
        .collect_vec();
    let min_steps_to_e = coordinate_list_start
        .iter()
        .filter(|c| steps_from_e[c.0][c.1] != -1)
        .map(|c| steps_from_e[c.0][c.1])
        .min()
        .unwrap_or(-1);
    println!("a-E in {:?} min steps", min_steps_to_e);
}
