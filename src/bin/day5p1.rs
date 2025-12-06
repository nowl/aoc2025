use std::io;

use anyhow::Error;
use debug_print::debug_println;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    num = { ASCII_DIGIT+ }
    range = { num ~ "-" ~ num }
    ingredient = { num }
    data = { SOI ~ range ~ (WHITE_SPACE+ ~ range)* ~ WHITE_SPACE* ~ ingredient ~ (WHITE_SPACE+ ~ ingredient)* ~ WHITE_SPACE* ~ EOI}
"#]
struct PestParser;

#[derive(Debug)]
struct Data {
    ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

fn parse() -> Result<Data, Error> {
    let mut ranges = vec![];
    let mut ingredients = vec![];

    let input = io::read_to_string(io::stdin())?;
    let mut data = PestParser::parse(Rule::data, &input)?;
    for pair in data.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::range => {
                let mut pairs = pair.into_inner();
                let start = pairs.next().unwrap().as_str().parse::<u64>().unwrap();
                let end = pairs.next().unwrap().as_str().parse::<u64>().unwrap();
                ranges.push((start, end));
            }
            Rule::ingredient => {
                let mut pairs = pair.into_inner();
                let ingredient = pairs.next().unwrap().as_str().parse::<u64>().unwrap();
                ingredients.push(ingredient);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(Data {
        ranges,
        ingredients,
    })
}

fn in_range(x: u64, ranges: &Vec<(u64, u64)>) -> bool {
    ranges.iter().any(|&(start, end)| x >= start && x <= end)
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;

    debug_println!("{data:#?}");

    let mut count = 0;
    for &ingredient in data.ingredients.iter() {
        if in_range(ingredient, &data.ranges) {
            debug_println!("ingredient {ingredient} in range");
            count += 1;
        }
    }

    println!("{count}");

    Ok(())
}
