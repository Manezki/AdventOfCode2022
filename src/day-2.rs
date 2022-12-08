use std::{fs, path::Path};

fn rock_paper_scissors(left: &str, right: &str) -> i64 {
    match left {
        "A" => match right {
            "X" => return 3,
            "Y" => return 6,
            "Z" => return 0,
            _ => panic!("Unhandled right case, {:?}", right),
        },
        "B" => match right {
            "X" => return 0,
            "Y" => return 3,
            "Z" => return 6,
            _ => panic!("Unhandled right case, {:?}", right),
        },
        "C" => match right {
            "X" => return 6,
            "Y" => return 0,
            "Z" => return 3,
            _ => panic!("Unhandled right case, {:?}", right),
        },
        _ => panic!("Unhandled left case, {:?}", left),
    }
}

fn choose_action<'a>(left: &'a str, right: &'a str) -> &'a str {
    match left {
        "A" => match right {
            "X" => return "Z",
            "Y" => return "X",
            "Z" => return "Y",
            _ => panic!("Unhandled right case, {:?}", right),
        },
        "B" => match right {
            "X" => return "X",
            "Y" => return "Y",
            "Z" => return "Z",
            _ => panic!("Unhandled right case, {:?}", right),
        },
        "C" => match right {
            "X" => return "Y",
            "Y" => return "Z",
            "Z" => return "X",
            _ => panic!("Unhandled right case, {:?}", right),
        },
        _ => panic!("Unhandled left case, {:?}", left),
    }
}

fn main() {
    let input_path = Path::new("inputs/day-2.txt");
    let puzzle_input =
        fs::read_to_string(input_path).expect("Something went wrong reading the file");

    let rounds = puzzle_input.split("\n").map(|round| {
        if let Some((left, right)) = round.split_once(" ") {
            return (left, right);
        } else {
            panic!("Could not split the input string")
        }
    });

    let puzzle_1_score: i64 = rounds
        .clone()
        .map(|(l, r)| {
            let alt_score = (r.bytes().next().unwrap() - 87) as i64;
            return rock_paper_scissors(l, r) + alt_score;
        })
        .sum();

    println!("Puzzle 1 solution: {:#?}", puzzle_1_score);

    let puzzle_2_score: i64 = rounds
        .map(|(left, right)| {
            let act = choose_action(left, right);
            let alt_score = (act.bytes().next().unwrap() - 87) as i64;
            return rock_paper_scissors(left, act) + alt_score;
        })
        .sum();

    println!("Puzzle 2 solution: {:#?}", puzzle_2_score);
}
