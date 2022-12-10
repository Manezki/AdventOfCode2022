use std::{collections::VecDeque, fs, path::Path};

fn unique_window(input: &str, window_len: usize) -> usize {
    let mut window: VecDeque<char> = VecDeque::new();
    let mut counts: Vec<u8> = vec![0; 26];

    for (i, c) in input.chars().enumerate() {
        if window.len() < window_len {
            counts[c as usize - 97] += 1;
            window.push_back(c);
            continue;
        }

        // If all counts are 1, then we have found the marker
        let mut col = 0;
        for count in &counts {
            col = col | count;
        }
        if col == 1 {
            return i;
        }

        let popped = window.pop_front().unwrap();
        counts[popped as usize - 97] -= 1;
        window.push_back(c);
        counts[c as usize - 97] += 1;
    }
    // Puzzle should guarantee that this line is not necessari to reach
    return 0;
}

fn main() {
    let input_path = Path::new("inputs/day-6.txt");
    let puzzle_input =
        fs::read_to_string(input_path).expect("Something went wrong reading the file");

    println!(
        "Puzzle 1 solution: {:#?}",
        unique_window(puzzle_input.as_str(), 4)
    );
    println!(
        "Puzzle 2 solution: {:#?}",
        unique_window(puzzle_input.as_str(), 14)
    );
}

#[test]
fn examples_puzzle_1() {
    struct TestCase<'a> {
        input: &'a str,
        want: usize,
    }

    let test_cases = vec![
        TestCase {
            input: "bvwbjplbgvbhsrlpgdmjqwftvncz",
            want: 5,
        },
        TestCase {
            input: "nppdvjthqldpwncqszvftbrmjlhg",
            want: 6,
        },
        TestCase {
            input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            want: 10,
        },
        TestCase {
            input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
            want: 11,
        },
    ];

    for tc in test_cases {
        assert_eq!(unique_window(&tc.input, 4), tc.want);
    }
}

#[test]
fn examples_puzzle_2() {
    struct TestCase<'a> {
        input: &'a str,
        want: usize,
    }

    let test_cases = vec![
        TestCase {
            input: "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            want: 19,
        },
        TestCase {
            input: "bvwbjplbgvbhsrlpgdmjqwftvncz",
            want: 23,
        },
        TestCase {
            input: "nppdvjthqldpwncqszvftbrmjlhg",
            want: 23,
        },
        TestCase {
            input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            want: 29,
        },
        TestCase {
            input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
            want: 26,
        },
    ];

    for tc in test_cases {
        assert_eq!(unique_window(&tc.input, 14), tc.want);
    }
}
