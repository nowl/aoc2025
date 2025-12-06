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

fn start_range_trim(x: u64, ranges: &Vec<(u64, u64)>) -> Option<u64> {
    // if x is in another range, then return value to trim to
    for &(start, end) in ranges.iter() {
        if x > start && x <= end {
            return Some(end + 1);
        }
    }
    None
}

fn end_range_trim(x: u64, ranges: &Vec<(u64, u64)>) -> Option<u64> {
    // if x is in another range, then return value to trim to
    for &(start, end) in ranges.iter() {
        if x >= start && x < end {
            return Some(start - 1);
        }
    }
    None
}

fn full_overlap(x: u64, y: u64, ranges: &Vec<(u64, u64)>) -> bool {
    let mut found = false;
    for &(start, end) in ranges.iter() {
        if x == start && y == end {
            if found {
                return true;
            }
            found = true;
        }
    }
    false
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;

    //debug_println!("{data:#?}");

    let mut ranges = data.ranges;

    // trim ranges
    loop {
        let mut found_overlap = false;
        let ranges_tmp = ranges.clone();
        for (start, end) in ranges.iter_mut() {
            if start > end {
                continue;
            }
            if let Some(new_start) = start_range_trim(*start, &ranges_tmp) {
                debug_println!(
                    "found start overlap with range: {:?}, new range: {:?}",
                    (*start, *end),
                    (new_start, *end)
                );
                *start = new_start;
                found_overlap = true;
                break;
            }
            if let Some(new_end) = end_range_trim(*end, &ranges_tmp) {
                debug_println!(
                    "found end overlap with range: {:?}, new range: {:?}",
                    (*start, *end),
                    (*start, new_end)
                );
                *end = new_end;
                found_overlap = true;
                break;
            }
            if full_overlap(*start, *end, &ranges_tmp) {
                let new_start = *end + 1;
                debug_println!(
                    "full overlap with range: {:?}, new range: {:?}",
                    (*start, *end),
                    (new_start, *end)
                );
                *start = new_start;
                found_overlap = true;
                break;
            }
        }
        if !found_overlap {
            break;
        }
    }

    debug_println!("{ranges:#?}");

    let mut count = 0;
    for (start, end) in ranges {
        if end >= start {
            count += end - start + 1;
        }
    }

    println!("{count}");

    Ok(())
}
