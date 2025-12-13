use std::io;

use anyhow::Error;
use debug_print::debug_println;
use itertools::Itertools;
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r##"
    space = _{ " " }
    index = { ASCII_DIGIT+ }
    shapepart = { "#" | "." }
    shapeline = { shapepart+ }
    shape = { index ~ ":" ~ WHITE_SPACE+ ~ shapeline ~ (WHITE_SPACE+ ~ shapeline)* }
    shapes = { shape ~ (WHITE_SPACE+ ~ shape)* }
    width = { ASCII_DIGIT+ }
    length = { ASCII_DIGIT+ }
    quantities = { ASCII_DIGIT+ ~ (space+ ~ ASCII_DIGIT+)* }
    region = { width ~ "x" ~ length ~ ":" ~ WHITE_SPACE* ~ quantities }
    regions = { region ~ (WHITE_SPACE+ ~ region)* }
    data = { SOI ~ shapes ~ WHITE_SPACE+ ~ regions ~ WHITE_SPACE* ~ EOI}
"##]
struct PestParser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Shape {
    index: i32,
    shape: [bool; 9],
}

#[derive(Debug)]
struct Region {
    width: usize,
    length: usize,
    quantities: Vec<i32>,
}

#[derive(Debug)]
struct Data {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

fn parse_shape(pair: &Pair<'_, Rule>) -> Shape {
    let mut index = 0;
    let mut shape = [false; 9];
    let mut row = 0;
    for pair in pair.clone().into_inner() {
        match pair.as_rule() {
            Rule::index => {
                index = pair.as_str().parse().unwrap();
            }
            Rule::shapeline => {
                for (col, val) in pair.into_inner().enumerate() {
                    match val.as_str() {
                        "#" => {
                            shape[row * 3 + col] = true;
                        }
                        "." => (),
                        _ => unreachable!(),
                    }
                }
                row += 1;
            }
            _ => unreachable!(),
        }
    }
    Shape { index, shape }
}

fn parse_region(pair: &Pair<'_, Rule>) -> Region {
    let mut quantities = vec![];
    let mut width = 0;
    let mut length = 0;
    for pair in pair.clone().into_inner() {
        match pair.as_rule() {
            Rule::width => {
                width = pair.as_str().parse().unwrap();
            }
            Rule::length => {
                length = pair.as_str().parse().unwrap();
            }
            Rule::quantities => {
                quantities = pair
                    .as_str()
                    .split_ascii_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect();
            }
            _ => unreachable!(),
        }
    }
    Region {
        width,
        length,
        quantities,
    }
}

fn parse() -> Result<Data, Error> {
    let mut shapes = vec![];
    let mut regions = vec![];

    let input = io::read_to_string(io::stdin())?;
    let mut data = PestParser::parse(Rule::data, &input)?;
    for pair in data.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::shapes => {
                for pair in pair.into_inner() {
                    shapes.push(parse_shape(&pair));
                }
            }
            Rule::regions => {
                for pair in pair.into_inner() {
                    regions.push(parse_region(&pair));
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(Data { shapes, regions })
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;
    debug_println!("{data:#?}");

    let area_shapes = data
        .shapes
        .iter()
        .map(|v| v.shape.iter().filter(|&v| *v).count() as i32)
        .collect_vec();

    let mut count = 0;
    for region in data.regions {
        let area_shapes = region
            .quantities
            .iter()
            .enumerate()
            .map(|(n, count)| {
                let shape_area = area_shapes[n];
                shape_area * count
            })
            .sum::<i32>();
        let area_board = region.width as i32 * region.length as i32;
        if area_shapes <= area_board {
            count += 1;
        }
    }

    println!("{count}");

    Ok(())
}
