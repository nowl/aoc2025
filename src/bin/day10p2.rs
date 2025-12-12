use std::{i32, io, ops::RangeInclusive, vec};

use anyhow::Error;
use debug_print::debug_println;
use itertools::Itertools;
use nalgebra::{DMatrix, DVector};
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

type IndexType = [u16; 20];

#[derive(Debug, Default)]
struct Constraint {
    indices: IndexType,
    total: u16,
}

fn generate_possible_for_idx(
    idx: usize,
    values: &IndexType,
    constraints: &Vec<Constraint>,
) -> RangeInclusive<u16> {
    let mut min = 0;
    let mut max = 500;

    debug_assert!(values[idx] == 0);

    for c in constraints.iter() {
        let mut this_max = c.total;
        for (n, val) in values.iter().enumerate() {
            if n == idx {
                continue;
            }

            if c.indices[idx] == 1 && c.indices[n] == 1 {
                this_max -= val;
                max = max.min(this_max);
            }
        }

        if c.indices[idx] == 1 && c.indices[idx + 1..].iter().all(|v| *v == 0) {
            let mut this_min = c.total;
            for (n, val) in values[0..idx].iter().enumerate() {
                if c.indices[n] == 1 {
                    this_min -= val;
                }
            }

            min = min.max(this_min);
        }
    }

    min..=max
}

fn recursive_search(
    idx: usize,
    current_values: IndexType,
    constraints: &Vec<Constraint>,
    a: &DMatrix<u16>,
    b: &DVector<u16>,
) -> i32 {
    let num_cols = a.shape().1;

    // test if we have reached an answer
    let x = DVector::from_column_slice(&current_values[0..num_cols]);
    let mut best = if a * &x == *b {
        x.sum() as i32
    } else {
        i32::MAX
    };

    // exit if search too deep
    if idx >= num_cols {
        return best;
    }

    let possible = generate_possible_for_idx(idx, &current_values, &constraints);
    for p in possible {
        let mut values = current_values.clone();
        values[idx] = p;
        let potential_best = recursive_search(idx + 1, values, constraints, a, b);
        best = best.min(potential_best);
    }
    best
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;

    debug_println!("{data:#?}");

    let mut count = 0;
    for puzzle in data.puzzles.iter() {
        let bvals = puzzle.joltage.iter().map(|v| *v as u16).collect_vec();

        // optimization to sort buttons by len, this helps to constrain min/max later
        let buttons = puzzle
            .buttons
            .iter()
            .sorted_by_key(|v| v.len())
            .rev()
            .collect_vec();

        debug_println!("{:?}", buttons);

        let b = DVector::from_vec(bvals);
        let a = DMatrix::from_fn(puzzle.joltage.len(), buttons.len(), |r, c| {
            let button = &buttons[c];
            let idx = r as i8;
            if button.contains(&idx) { 1u16 } else { 0u16 }
        });

        debug_println!("{}", b);
        debug_println!("{}", a);

        let constraints = (0..a.shape().0)
            .map(|row| {
                let mut constraint = Constraint::default();
                constraint.total = b[row];
                for col in 0..a.shape().1 {
                    if *a.get((row, col)).unwrap() == 1 {
                        constraint.indices[col] = 1;
                    }
                }
                debug_println!("constraint: {:?}", constraint);
                constraint
            })
            .collect_vec();

        let values = IndexType::default();
        let best = recursive_search(0, values, &constraints, &a, &b);
        debug_println!("{:?}", best);

        println!("{:?}", best);

        count += best;
    }

    println!("final sum: {count}");

    Ok(())
}
