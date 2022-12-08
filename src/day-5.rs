use std::{
    collections::{LinkedList, VecDeque},
    fs,
    path::Path,
};

use regex::Regex;

#[derive(Debug)]
struct Stacks {
    stacks: Vec<LinkedList<char>>,
}

impl Stacks {
    pub fn new(n_stacks: usize, crates: &Vec<&str>) -> Self {
        let mut stacks = Stacks {
            stacks: vec![LinkedList::new(); n_stacks],
        };
        crates.iter().for_each(|row| {
            let mut idx = 0;
            loop {
                let potential_create = &row[idx..idx + 3];
                if potential_create.starts_with("[") {
                    stacks.stacks[(idx / 4)]
                        .push_front(potential_create.chars().collect::<Vec<char>>()[1]);
                }
                if row.len() <= idx + 4 {
                    break;
                }
                idx += 4;
            }
        });
        stacks
    }

    fn move_single(&mut self, from: usize, to: usize) {
        if let Some(elem) = self.stacks[from - 1].pop_back() {
            self.stacks[to - 1].push_back(elem);
            return;
        }
    }

    fn move_n(&mut self, n: usize, from: usize, to: usize) {
        let mut tmp = VecDeque::new();

        for _ in 0..n {
            tmp.push_front(self.stacks[from - 1].pop_back().unwrap());
        }
        for _ in 0..n {
            self.stacks[to - 1].push_back(tmp.pop_front().unwrap());
        }
    }

    fn code(&self) -> String {
        return self
            .stacks
            .iter()
            .map(|stack| {
                return stack.back().unwrap().to_string();
            })
            .collect::<Vec<String>>()
            .join("");
    }
}

fn main() {
    let input_path = Path::new("inputs/day-5.txt");
    let puzzle_input =
        fs::read_to_string(input_path).expect("Something went wrong reading the file");
    let split_input = puzzle_input.split("\n");

    let crates = split_input
        .clone()
        .take_while(|s| s.contains("["))
        .collect::<Vec<&str>>();
    let stack_names = split_input.clone().nth(crates.len()).unwrap();
    let mut moves = split_input.skip(crates.len() + 2);

    let n_stacks = stack_names.split_whitespace().collect::<Vec<&str>>().len();
    let mut puzzle_1_stacks = Stacks::new(n_stacks, &crates);
    let mut puzzle_2_stacks = Stacks::new(n_stacks, &crates);

    let move_regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    while let Some(mv) = moves.next() {
        let capts = move_regex.captures(mv).unwrap();

        let n = usize::from_str_radix(capts.get(1).unwrap().as_str(), 10).unwrap();
        let from = usize::from_str_radix(capts.get(2).unwrap().as_str(), 10).unwrap();
        let to = usize::from_str_radix(capts.get(3).unwrap().as_str(), 10).unwrap();

        for _ in 0..n {
            puzzle_1_stacks.move_single(from, to);
        }
        puzzle_2_stacks.move_n(n, from, to);
    }

    println!("Puzzle 1 solution: {:#?}", puzzle_1_stacks.code());
    println!("Puzzle 2 solution: {:#?}", puzzle_2_stacks.code());
}
