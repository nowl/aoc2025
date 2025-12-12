use std::{collections::HashMap, io};

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

fn count_all_paths<'a>(
    state @ (input, hit_dac, hit_fft): (&str, bool, bool),
    data: &'a Data,
    memo: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    if let Some(count) = memo.get(&state) {
        return *count;
    }
    if input == "out" {
        if hit_dac && hit_fft {
            return 1;
        } else {
            return 0;
        }
    }

    let mut count = 0;
    for output in data.junctions.get(input).unwrap() {
        let hit_dac = hit_dac || output == "dac";
        let hit_fft = hit_fft || output == "fft";
        let new_state = (output.as_str(), hit_dac, hit_fft);
        let this_count = count_all_paths(new_state, data, memo);
        memo.insert(new_state, this_count);
        count += this_count;
    }

    count
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;
    debug_println!("{data:#?}");

    let mut memo = HashMap::new();
    let count = count_all_paths(("svr", false, false), &data, &mut memo);
    println!("{count}");

    Ok(())
}
