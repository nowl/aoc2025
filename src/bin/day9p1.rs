use std::{io, vec};

use anyhow::Error;
use debug_print::debug_println;
use itertools::Itertools;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    num = { ASCII_DIGIT+ }
    row = { num ~ "," ~ num }
    data = { SOI ~ row ~ (WHITE_SPACE+ ~ row)* ~ WHITE_SPACE* ~ EOI}
"#]
struct PestParser;

#[derive(Debug)]
struct Data {
    rows: Vec<(i32, i32)>,
}

fn parse() -> Result<Data, Error> {
    let mut rows = vec![];

    let input = io::read_to_string(io::stdin())?;
    let mut data = PestParser::parse(Rule::data, &input)?;
    for pair in data.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::row => {
                let mut pairs = pair.into_inner();
                let x = pairs.next().unwrap().as_str().parse().unwrap();
                let y = pairs.next().unwrap().as_str().parse().unwrap();
                rows.push((x, y));
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(Data { rows })
}

fn calc_area((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> i64 {
    ((x1 - x2).abs() + 1) as i64 * ((y1 - y2).abs() + 1) as i64
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;

    debug_println!("{data:#?}");

    let result = data
        .rows
        .iter()
        .combinations(2)
        .map(|v| calc_area(v[0], v[1]))
        .max()
        .unwrap();

    println!("{result}");
    Ok(())
}
