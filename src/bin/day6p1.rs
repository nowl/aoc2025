use std::io;

use anyhow::Error;
use debug_print::debug_println;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    SPACE = _{ " " | "\\t" }
    num = { ASCII_DIGIT+ }
    op = { "*" | "+" }
    num_row = { num ~ (SPACE+ ~ num)* ~ WHITE_SPACE+ }
    op_row = { op ~ (SPACE+ ~ op)* ~ WHITE_SPACE+ }
    data = { SOI ~ num_row+ ~ op_row ~ WHITE_SPACE* ~ EOI}
"#]
struct PestParser;

#[derive(Debug)]
enum Op {
    ADD,
    MULT,
}

#[derive(Debug)]
struct Data {
    nums: Vec<Vec<i32>>,
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
                let pairs = pair.into_inner();
                let row = pairs
                    .into_iter()
                    .map(|v| v.as_str().parse::<i32>().unwrap())
                    .collect();
                nums.push(row);
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

pub fn main() -> Result<(), Error> {
    let data = parse()?;

    debug_println!("{data:#?}");

    // check lengths
    data.nums.iter().fold(None, |acc, row| {
        if let Some(len) = acc {
            assert!(row.len() == len);
            acc
        } else {
            Some(row.len())
        }
    });
    assert!(data.ops.len() == data.nums[0].len());

    let num_rows = data.nums.len();
    let num_values = data.nums[0].len();

    let mut result = 0;
    for col in 0..num_values {
        let mut tmp = match data.ops[col] {
            Op::ADD => 0u64,
            Op::MULT => 1u64,
        };

        for row in 0..num_rows {
            match data.ops[col] {
                Op::ADD => tmp += data.nums[row][col] as u64,
                Op::MULT => tmp *= data.nums[row][col] as u64,
            }
        }

        result += tmp;
    }

    println!("{result}");

    Ok(())
}
