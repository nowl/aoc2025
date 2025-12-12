use std::{io, vec};

use anyhow::Error;
use aoc2025::dijkstra::{DijkstraConfig, DijkstraInput, DijkstraMap};
use debug_print::debug_println;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r##"
    state_val = { "." | "#" }
    state = { "[" ~ state_val+ ~ "]" }
    num = { ASCII_DIGIT+ }
    button = { "(" ~ num ~ ("," ~ num)* ~ ")" }
    buttons = { button ~ (WHITE_SPACE+ ~ button)* }
    joltage = { "{" ~ num ~ ("," ~ num)* ~ "}" }
    line = { state ~ WHITE_SPACE+ ~ buttons ~ WHITE_SPACE+ ~ joltage }
    data = { SOI ~ line ~ (WHITE_SPACE+ ~ line)* ~ WHITE_SPACE* ~ EOI}
"##]
struct PestParser;

#[derive(Debug, Default)]
struct Puzzle {
    goal: Vec<bool>,
    buttons: Vec<Vec<i8>>,
    joltage: Vec<i16>,
}

#[derive(Debug)]
struct Data {
    puzzles: Vec<Puzzle>,
}

fn parse() -> Result<Data, Error> {
    let input = io::read_to_string(io::stdin())?;
    let mut data = PestParser::parse(Rule::data, &input)?;
    let mut puzzles = vec![];
    for pair in data.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::line => {
                let mut puzzle = Puzzle::default();
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::state => {
                            let state = pair
                                .into_inner()
                                .map(|p| match p.as_str() {
                                    "." => false,
                                    "#" => true,
                                    _ => unimplemented!(),
                                })
                                .collect();
                            puzzle.goal = state;
                        }
                        Rule::joltage => {
                            let joltage = pair
                                .into_inner()
                                .map(|p| p.as_str().parse().unwrap())
                                .collect();
                            puzzle.joltage = joltage;
                        }
                        Rule::buttons => {
                            let mut buttons = vec![];
                            for pair in pair.into_inner() {
                                let button = pair
                                    .into_inner()
                                    .map(|p| p.as_str().parse().unwrap())
                                    .collect();
                                buttons.push(button);
                            }
                            puzzle.buttons = buttons;
                        }
                        _ => unreachable!(),
                    }
                }
                puzzles.push(puzzle);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(Data { puzzles })
}

fn init_i32_by_state(val: &Vec<bool>) -> i32 {
    let mut r = 0;
    for (n, v) in val.iter().enumerate() {
        if *v {
            r ^= 0x1 << n;
        }
    }
    r
}

fn toggle_i32_by_button(r: &mut i32, button: usize) {
    *r = (*r) ^ (0x1 << button);
}

impl DijkstraInput for Puzzle {
    type Cost = i32;
    type Index = i32;

    fn get_adjacent(&self, state: &Self::Index) -> Vec<(Self::Cost, Self::Index)> {
        self.buttons
            .iter()
            .map(|button| {
                let mut new_state = state.clone();
                for b in button {
                    let idx = *b as usize;
                    toggle_i32_by_button(&mut new_state, idx);
                }
                (1, new_state)
            })
            .collect()
    }
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;

    debug_println!("{data:#?}");

    let mut count = 0;
    for puzzle in data.puzzles.iter() {
        #[cfg(debug_assertions)]
        let config = DijkstraConfig { print_1000: true };
        #[cfg(not(debug_assertions))]
        let config = DijkstraConfig { print_1000: false };

        let mut dmap = DijkstraMap::new(puzzle, config);
        let goal_state = init_i32_by_state(&puzzle.goal);
        debug_println!("{:#?}", goal_state);
        let paths = dmap.run((0, 0));
        let path = DijkstraMap::<Puzzle, i32>::extract_path(&0, &goal_state, &paths);
        debug_println!("{:#?}", path);
        count += path.len();
    }

    println!("{count}");

    Ok(())
}
