use std::{collections::HashMap, io};

use anyhow::Error;
use debug_print::debug_println;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    start = { "S" }
    splitter = { "^" }
    empty = { "." }
    row = { (start | splitter | empty)+ }
    data = { SOI ~ row ~ (WHITE_SPACE ~ row)* ~ WHITE_SPACE ~ EOI}
"#]
struct PestParser;

#[derive(Debug, PartialEq, Eq)]
enum Pos {
    Start,
    Empty,
    Splitter,
}

#[derive(Debug)]
struct Data {
    board: HashMap<(i32, i32), Pos>,
}

fn parse() -> Result<Data, Error> {
    let mut board = HashMap::new();
    let mut row = 0;
    let mut col = 0;

    let input = io::read_to_string(io::stdin())?;
    let mut data = PestParser::parse(Rule::data, &input)?;
    for pair in data.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::row => {
                for pairs in pair.into_inner() {
                    let spot = match pairs.as_rule() {
                        Rule::splitter => Pos::Splitter,
                        Rule::empty => Pos::Empty,
                        Rule::start => Pos::Start,
                        _ => unreachable!(),
                    };
                    board.insert((col, row), spot);
                    col += 1;
                }
                col = 0;
                row += 1;
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(Data { board })
}

impl Data {
    fn start_pos(&self) -> (i32, i32) {
        self.board
            .iter()
            .find(|&(_, val)| *val == Pos::Start)
            .map(|(pos, _)| pos.clone())
            .unwrap()
    }
}

fn fire_tachyon(
    (col, mut row): (i32, i32),
    board: &HashMap<(i32, i32), Pos>,
    memo: &mut HashMap<(i32, i32), u64>,
) -> u64 {
    let new_positions;
    loop {
        let obj = board.get(&(col, row));
        // check if out of bounds
        if obj.is_none() {
            return 1;
        } else {
            match obj.unwrap() {
                Pos::Start => unreachable!(),
                Pos::Empty => {
                    row += 1;
                }
                Pos::Splitter => {
                    new_positions = Some(((col - 1, row), (col + 1, row)));
                    break;
                }
            }
        }
    }

    let left = new_positions.unwrap().0;
    let right = new_positions.unwrap().1;

    if !memo.contains_key(&left) {
        let v = fire_tachyon(left, board, memo);
        memo.insert(left, v);
    }

    if !memo.contains_key(&right) {
        let v = fire_tachyon(right, board, memo);
        memo.insert(right, v);
    }

    return memo.get(&left).unwrap() + memo.get(&right).unwrap();
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;

    debug_println!("{data:#?}");

    let (start_col, start_row) = data.start_pos();
    let mut memoized_splitter_count = HashMap::new();
    let count = fire_tachyon(
        (start_col, start_row + 1),
        &data.board,
        &mut memoized_splitter_count,
    );

    println!("{count}");

    Ok(())
}
