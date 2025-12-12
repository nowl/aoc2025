use std::{
    collections::{HashMap, VecDeque},
    io,
};

use anyhow::Error;
use debug_print::debug_println;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r##"
    space = _{ " " }
    device = { ASCII_ALPHA_LOWER{3} }
    outputs = { device ~ (space+ ~ device)* }
    line = { device ~ ":" ~ space+ ~ outputs }
    data = { SOI ~ line ~ (WHITE_SPACE+ ~ line)* ~ WHITE_SPACE* ~ EOI}
"##]
struct PestParser;

#[derive(Debug)]
struct Data {
    junctions: HashMap<String, Vec<String>>,
}

fn parse() -> Result<Data, Error> {
    let input = io::read_to_string(io::stdin())?;
    let mut data = PestParser::parse(Rule::data, &input)?;
    let mut junctions: HashMap<_, Vec<_>> = HashMap::new();
    for pair in data.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::line => {
                let mut input = None;
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::device => {
                            input = Some(pair.as_str());
                        }
                        Rule::outputs => {
                            for pair in pair.into_inner() {
                                junctions
                                    .entry(input.unwrap().to_string())
                                    .or_default()
                                    .push(pair.as_str().to_string());
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(Data { junctions })
}

fn count_all_paths(start: &str, end: &str, data: &Data) -> i32 {
    let mut queue = VecDeque::new();
    let mut count = 0;

    queue.push_back(start);

    while let Some(input) = queue.pop_front() {
        if input == end {
            count += 1;
        } else {
            data.junctions.get(input).unwrap().iter().for_each(|i| {
                queue.push_back(i);
            });
        }
    }
    count
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;
    debug_println!("{data:#?}");

    let count = count_all_paths("you", "out", &data);
    println!("{count}");

    Ok(())
}
