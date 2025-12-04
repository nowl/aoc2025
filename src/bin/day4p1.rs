use std::{collections::HashMap, io};

use anyhow::Error;
use debug_print::debug_println;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    roll = { "@" }
    floor = { "." }
    row = { (roll | floor)+ }
    data = { SOI ~ row ~ (WHITE_SPACE+ ~ row)* ~ WHITE_SPACE* ~ EOI}
"#]
struct PestParser;

#[derive(Debug, PartialEq, Eq)]
enum Spot {
    Roll,
    Floor,
}

#[derive(Debug)]
struct Data {
    spots: HashMap<(i32, i32), Spot>,
}

fn parse() -> Result<Data, Error> {
    let mut spots = HashMap::new();
    let mut row = 0;
    let mut col = 0;

    let input = io::read_to_string(io::stdin())?;
    let mut data = PestParser::parse(Rule::data, &input)?;
    for pair in data.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::row => {
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::roll => {
                            spots.insert((col, row), Spot::Roll);
                            col += 1;
                        }
                        Rule::floor => {
                            spots.insert((col, row), Spot::Floor);
                            col += 1;
                        }
                        _ => unreachable!(),
                    }
                }
                row += 1;
                col = 0;
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(Data { spots })
}

impl Data {
    fn dims(&self) -> (usize, usize) {
        let cols = self.spots.keys().map(|x| x.0).max().unwrap() + 1;
        let rows = self.spots.keys().map(|x| x.1).max().unwrap() + 1;
        (cols as usize, rows as usize)
    }

    fn is_floor(&self, pos: (i32, i32)) -> bool {
        self.spots.get(&pos).map_or(false, |x| *x == Spot::Floor)
    }

    fn is_roll(&self, pos: (i32, i32)) -> bool {
        self.spots.get(&pos).map_or(false, |x| *x == Spot::Roll)
    }

    fn is_empty(&self, pos: (i32, i32)) -> bool {
        self.spots.get(&pos).is_none()
    }

    fn can_access(&self, pos: (i32, i32)) -> bool {
        let mut count = 0;
        for dcol in [-1, 0, 1] {
            for drow in [-1, 0, 1] {
                if dcol == 0 && drow == 0 {
                    continue;
                }

                let dpos = (pos.0 + dcol, pos.1 + drow);
                if self.is_floor(dpos) || self.is_empty(dpos) {
                    count += 1;
                }
            }
        }
        count >= 5
    }
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;

    //debug_println!("{data:#?}");

    let mut count = 0;
    let (ncols, nrows) = data.dims();
    for row in 0..nrows {
        for col in 0..ncols {
            let pos = (col as i32, row as i32);
            if !data.is_roll(pos) {
                continue;
            }
            let can_access = data.can_access(pos);
            debug_println!("col: {col}, row: {row}, can access: {can_access}");
            if can_access {
                count += 1;
            }
        }
    }

    println!("{count}");

    Ok(())
}
