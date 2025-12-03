use std::io;

use anyhow::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    bank = { ASCII_DIGIT+ }
    data = { SOI ~ bank ~ (WHITE_SPACE+ ~ bank)* ~ WHITE_SPACE* ~ EOI}
"#]
struct PestParser;

#[derive(Debug)]
struct Bank(Vec<u8>);

#[derive(Debug)]
struct Data {
    banks: Vec<Bank>,
}

fn parse() -> Result<Data, Error> {
    let mut banks = vec![];

    let input = io::read_to_string(io::stdin())?;
    let mut data = PestParser::parse(Rule::data, &input)?;
    for pair in data.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::bank => {
                let bank = pair.as_str();
                let nums = bank
                    .chars()
                    .map(|c| c.to_string().parse::<u8>())
                    .collect::<Result<Vec<_>, _>>()?;
                let bank = Bank(nums);
                banks.push(bank);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(Data { banks })
}

impl Bank {
    fn find_largest(&self) -> Vec<u8> {
        let mut result = vec![];
        let b = &self.0;
        let m1 = b[0..b.len() - 1]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|v| v.1)
            .unwrap();
        result.push(m1.1.clone());

        let m2 = b[m1.0 + 1..]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|v| v.1)
            .unwrap();
        result.push(m2.1.clone());

        result
    }
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;
    let mut result = 0;
    for bank in data.banks {
        let max = bank.find_largest();
        let max = max
            .into_iter()
            .map(|x| x.to_string())
            .collect::<String>()
            .parse::<i32>()?;

        result += max;
    }

    println!("{result}");

    Ok(())
}
