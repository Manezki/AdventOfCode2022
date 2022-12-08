use std::{fs, path::Path};

#[derive(Debug)]
struct Elf {
    lower: i64,
    upper: i64,
}

impl std::fmt::Display for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "({}-{})", self.lower, self.upper);
    }
}

struct Pairs(Vec<(Elf, Elf)>);

impl std::fmt::Display for Pairs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.iter().fold(Ok(()), |result, pair| {
            result.and_then(|_| writeln!(f, "[{},{}]", pair.0, pair.1))
        })
    }
}

fn main() {
    let input_path = Path::new("inputs/day-4.txt");
    let puzzle_input =
        fs::read_to_string(input_path).expect("Something went wrong reading the file");

    let pairs = puzzle_input
        .split("\n")
        .map(|pair| {
            if let Some((left, right)) = pair.split_once(",") {
                if let Some((low, up)) = left.split_once("-") {
                    let left_elf = Elf {
                        lower: i64::from_str_radix(low, 10).unwrap(),
                        upper: i64::from_str_radix(up, 10).unwrap(),
                    };

                    if let Some((low, up)) = right.split_once("-") {
                        let right_elf = Elf {
                            lower: i64::from_str_radix(low, 10).unwrap(),
                            upper: i64::from_str_radix(up, 10).unwrap(),
                        };

                        return (left_elf, right_elf);
                    }
                }
            }
            panic!("Could not split a pair");
        })
        .collect::<Vec<(Elf, Elf)>>();

    let complete_overlap = pairs.iter().map(|(l, r)| {
        if l.lower >= r.lower && l.upper <= r.upper {
            return 1;
        }
        if r.lower >= l.lower && r.upper <= l.upper {
            return 1;
        }
        return 0;
    });

    println!("Puzzle 1 solution: {:#?}", complete_overlap.sum::<i64>());

    let partial_overlap = pairs.iter().map(|(l, r)| {
        if l.upper < r.lower {
            return 0;
        }
        if l.lower > r.upper {
            return 0;
        }
        return 1;
    });

    println!("Puzzle 2 solution: {:#?}", partial_overlap.sum::<i64>());
}
