use std::{fs, path::Path};

fn main() {
    let input_path = Path::new("inputs/day-1.txt");
    let puzzle_input =
        fs::read_to_string(input_path).expect("Something went wrong reading the file");

    let mut elves = puzzle_input
        .split("\n\n")
        .map(|elf| {
            let entries = elf.split("\n").map(|ration| {
                if ration == "" {
                    return 0;
                }
                return i64::from_str_radix(ration, 10).expect("Error parsing input value");
            });
            let total_calories: i64 = entries.sum();
            return total_calories;
        })
        .collect::<Vec<i64>>();
    println!("Puzzle 1 solution: {:#?}", elves.clone().into_iter().max());
    elves.sort();
    elves.reverse();
    println!(
        "Puzzle 2 solution: {:#?}",
        elves[..3].into_iter().sum::<i64>()
    );
}
