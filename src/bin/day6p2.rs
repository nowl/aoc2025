use std::{collections::HashMap, io, vec};

use anyhow::Error;
use debug_print::debug_println;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    SPACE = { " " }
    op = { "*" | "+" }
    num_row = @{ (ASCII_DIGIT | SPACE)+}
    op_row = { op ~ (WHITE_SPACE+ ~ op)* ~ WHITE_SPACE+ }
    data = { SOI ~ (num_row ~ NEWLINE)+ ~ op_row ~ EOI}
"#]
struct PestParser;

#[derive(Debug)]
enum Op {
    ADD,
    MULT,
}

#[derive(Debug)]
struct Data {
    nums: Vec<String>,
    ops: Vec<Op>,
}

fn parse() -> Result<Data, Error> {
    let mut nums = vec![];
    let mut ops = vec![];

    let input = io::read_to_string(io::stdin())?;
    let mut data = PestParser::parse(Rule::data, &input)?;
    for pair in data.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::num_row => {
                nums.push(pair.as_str().to_string());
            }
            Rule::op_row => {
                let pairs = pair.into_inner();
                ops = pairs
                    .into_iter()
                    .map(|v| match v.as_str() {
                        "*" => Op::MULT,
                        "+" => Op::ADD,
                        _ => unreachable!(),
                    })
                    .collect();
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(Data { nums, ops })
}

fn find_breaks(
    num_rows: usize,
    num_cols: usize,
    nums: &HashMap<(usize, usize), Option<char>>,
) -> Vec<usize> {
    let mut breaks = vec![];
    for col in 0..num_cols {
        if (0..num_rows).all(|row| nums.get(&(col, row)).unwrap().is_none()) {
            breaks.push(col);
        }
    }
    breaks
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;

    // check lengths
    data.nums.iter().fold(None, |acc, row| {
        if let Some(len) = acc {
            assert!(row.len() == len);
            acc
        } else {
            Some(row.len())
        }
    });

    let num_rows = data.nums.len();
    let num_cols = data.nums[0].len();

    let mut nums = HashMap::new();
    for row in 0..num_rows {
        for col in 0..num_cols {
            let pos = (col, row);
            let val = match data.nums[row].chars().nth(col).unwrap() {
                c @ '0'..='9' => Some(c),
                ' ' => None,
                _ => unreachable!(),
            };
            nums.insert(pos, val);
        }
    }

    debug_println!("{nums:#?}");

    let breaks = find_breaks(num_rows, num_cols, &nums);
    assert!(breaks.len() == data.ops.len() - 1);
    debug_println!("{breaks:#?}");

    let ranges = {
        let r = breaks.into_iter().fold((0, vec![]), |(start, mut acc), b| {
            acc.push((start, b));
            (b + 1, acc)
        });
        let mut ranges = r.1;
        ranges.push((r.0, num_cols));
        ranges
    };
    assert!(ranges.len() == data.ops.len());
    debug_println!("{ranges:#?}");

    let mut count = 0;
    for (coln, &(col_start, col_end)) in ranges.iter().enumerate() {
        let mut col_nums = vec![];
        for col in col_start..col_end {
            let mut num_string = String::new();
            for row in 0..num_rows {
                if let Some(c) = nums.get(&(col, row)).unwrap() {
                    num_string.push(*c);
                }
            }
            let col_num = num_string.parse::<i32>().unwrap();
            col_nums.push(col_num);
            debug_println!("{col_num}");
        }

        let val = match data.ops[coln] {
            Op::ADD => col_nums.iter().fold(0u64, |acc, n| acc + (*n as u64)),
            Op::MULT => col_nums.iter().fold(1u64, |acc, n| acc * (*n as u64)),
        };

        count += val;
    }

    println!("{count}");

    Ok(())
}
