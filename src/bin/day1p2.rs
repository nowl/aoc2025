use std::io;

use anyhow::Error;
use debug_print::debug_println;
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
    fn apply(&mut self, c: Comb) -> usize {
        let mut zero_passes = 0;
        let mut p = self.pos as i32;
        match c {
            Comb::Left(n) => {
                debug_println!("Left {n}");
                let tmp = p - n as i32;
                debug_println!("tmp: {tmp}");
                if tmp < 0 {
                    zero_passes += tmp.div_euclid(-100) as usize;

                    if p == 0 {
                        zero_passes -= 1;
                    }
                }

                p = tmp.mod_floor(&100);

                if p == 0 {
                    zero_passes += 1;
                }
                debug_println!("zero_passes: {zero_passes}");
            }
            Comb::Right(n) => {
                debug_println!("Right {n}");
                let tmp = p + n as i32;
                debug_println!("tmp: {tmp}");
                if tmp >= 100 {
                    zero_passes += tmp.div_euclid(100) as usize;
                }
                debug_println!("zero_passes: {zero_passes}");
                p = tmp.mod_floor(&100)
            }
        }
        self.pos = p as usize;
        zero_passes
    }
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;
    let mut state = State { pos: 50 };

    let mut count = 0;
    for comb in data.combinations {
        count += state.apply(comb);
    }

    println!("{}", count);

    Ok(())
}
