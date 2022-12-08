use std::{fs, path::Path};

fn item_priority(c: char) -> i64 {
    let priority;
    if (c as u32) < 91 {
        // Uppercase byte
        // Priority for uppercase
        priority = c as u32 - 65 + 27;
    } else {
        // Priority for lowercase
        priority = c as u32 - 97 + 1;
    }
    assert!(priority <= 52);
    return priority as i64;
}

fn main() {
    let input_path = Path::new("inputs/day-3.txt");
    let puzzle_input =
        fs::read_to_string(input_path).expect("Something went wrong reading the file");

    let rucksacks = puzzle_input.split("\n");
    let compartments = rucksacks.clone().map(|sack| {
        let compartment_size = sack.len() / 2;
        let (left, right) = sack.split_at(compartment_size);
        return (left, right);
    });

    let shared_items = compartments.map(|(l, r)| {
        for c in l.chars() {
            if r.contains(c) {
                return c;
            }
        }
        panic!("Could not find overlap between compartments");
    });

    let puzzle_1_priority: i64 = shared_items.map(|item| return item_priority(item)).sum();

    println!("Puzzle 1 solution: {:#?}", puzzle_1_priority);

    let mut badge_priorities: Vec<i64> = Vec::new();
    let mut groupable_rucksacks = rucksacks.clone();

    while let Some(elf_1) = groupable_rucksacks.next() {
        let elf_2 = groupable_rucksacks.next();
        let elf_3 = groupable_rucksacks.next();

        for c in elf_1.chars() {
            if elf_2.unwrap().contains(c) {
                if elf_3.unwrap().contains(c) {
                    badge_priorities.push(item_priority(c));
                    break;
                }
            }
        }
    }

    println!(
        "Puzzle 2 solution: {:#?}",
        badge_priorities.iter().sum::<i64>()
    );
}
