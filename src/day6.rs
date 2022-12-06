use itertools::Itertools;

fn main() {
    let input = include_str!("../input/day6.txt").trim();
    let examples_1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
";
    let find_distinct_substring = |s: &str, l: usize| {
        for i in l..s.len() {
            if *s
                .chars()
                .skip(i - l)
                .take(l)
                .counts()
                .values()
                .max()
                .unwrap()
                == 1
            {
                return i;
            }
        }
        0
    };
    let results_1 = (String::from(examples_1) + input)
        .lines()
        .map(|s| find_distinct_substring(s, 4))
        .collect_vec();
    // Verify using examples
    assert_eq!([7, 5, 6, 10, 11], results_1[..results_1.len() - 1]);
    println!(
        "First start-of-packet marker: {}",
        results_1.last().unwrap()
    );
    let examples_2 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
";
    let results_2 = (String::from(examples_2) + input)
        .lines()
        .map(|s| find_distinct_substring(s, 14))
        .collect_vec();
    // Verify using examples
    assert_eq!([19, 23, 23, 29, 26], results_2[..results_2.len() - 1]);
    println!(
        "First start-of-message marker: {}",
        results_2.last().unwrap()
    );
}
