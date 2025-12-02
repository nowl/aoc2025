use std::io;

use anyhow::Error;
use itertools::{FoldWhile, Itertools};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    begin = { ASCII_DIGIT+ }
    end = { ASCII_DIGIT+ }
    range = { begin ~ "-" ~ end }
    data = { SOI ~ range ~ ("," ~ range)* ~ WHITE_SPACE* ~ EOI}
"#]
struct PestParser;

#[derive(Debug)]
struct Range(usize, usize);

#[derive(Debug)]
struct Data {
    ranges: Vec<Range>,
}

fn parse() -> Result<Data, Error> {
    let mut ranges = vec![];

    let input = io::read_to_string(io::stdin())?;
    let mut data = PestParser::parse(Rule::data, &input)?;
    for pair in data.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::range => {
                let mut pairs = pair.into_inner();
                let start = pairs.next().unwrap().as_str().parse::<usize>().unwrap();
                let end = pairs.next().unwrap().as_str().parse::<usize>().unwrap();

                ranges.push(Range(start, end));
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(Data { ranges })
}

impl Data {
    fn in_range(&self, val: usize) -> bool {
        for range in self.ranges.iter() {
            if val >= range.0 && val <= range.1 {
                return true;
            }
        }
        false
    }

    fn out_of_range(&self, val: usize) -> bool {
        self.ranges.iter().all(|range| val > range.1)
    }
}

struct State {
    val: usize,
}

impl State {
    fn to_invalid(&self) -> usize {
        let mut s = String::new();
        let v = self.val.to_string();
        s.push_str(&v);
        s.push_str(&v);
        s.parse().unwrap()
    }
}

impl Iterator for State {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.to_invalid();
        self.val += 1;
        Some(v)
    }
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;

    let mut state = State { val: 1 };

    let result = state
        .fold_while(0, |acc, item| {
            if data.out_of_range(item) {
                FoldWhile::Done(acc)
            } else if data.in_range(item) {
                FoldWhile::Continue(acc + item)
            } else {
                FoldWhile::Continue(acc)
            }
        })
        .into_inner();

    println!("{result}");

    Ok(())
}
