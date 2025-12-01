use std::io;

use anyhow::Error;
use num::Integer;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    distance = { ASCII_DIGIT+ }
    left = { "L" ~ distance }
    right = { "R" ~ distance }
    code = { left | right }
    data = { SOI ~ (code ~ NEWLINE)+ ~ WHITE_SPACE* ~ EOI}
"#]
struct PestParser;

#[derive(Debug)]
enum Comb {
    Left(usize),
    Right(usize),
}

#[derive(Debug)]
struct Data {
    combinations: Vec<Comb>,
}

fn parse() -> Result<Data, Error> {
    let mut combinations = vec![];

    let input = io::read_to_string(io::stdin())?;
    let mut data = PestParser::parse(Rule::data, &input)?;
    for pair in data.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::code => {
                let pair = pair.into_inner().next().unwrap();
                match pair.as_rule() {
                    Rule::left => {
                        let distance = pair
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap();
                        combinations.push(Comb::Left(distance));
                    }
                    Rule::right => {
                        let distance = pair
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap();
                        combinations.push(Comb::Right(distance));
                    }
                    _ => unreachable!(),
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(Data { combinations })
}

#[derive(Debug)]
struct State {
    pos: usize,
}

impl State {
    fn apply(&mut self, c: Comb) {
        let mut p = self.pos as i32;
        match c {
            Comb::Left(n) => p = (p - n as i32).mod_floor(&100),
            Comb::Right(n) => p = (p + n as i32).mod_floor(&100),
        }
        self.pos = p as usize;
    }

    fn check(&self) -> bool {
        self.pos == 0
    }
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;
    let mut state = State { pos: 50 };

    let mut count = 0;
    for comb in data.combinations {
        state.apply(comb);
        if state.check() {
            count += 1;
        }
    }

    println!("{}", count);

    Ok(())
}
