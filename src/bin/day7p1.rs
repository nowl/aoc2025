use std::{
    collections::{HashMap, HashSet},
    io, vec,
};

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

struct Tachyon {
    is_active: bool,
    positions: HashSet<(i32, i32)>,
    cur_pos: (i32, i32),
}

struct Tachyons {
    tachyons: Vec<Tachyon>,
}

enum TachyonAction {
    OutOfBounds,
    SplitAt(i32, i32),
    MoveTo(i32, i32),
}

impl Tachyons {
    fn new(&mut self, pos: (i32, i32)) {
        if !self.any_at_position(&pos) {
            let mut positions = HashSet::new();
            positions.insert(pos.clone());
            self.tachyons.push(Tachyon {
                is_active: true,
                cur_pos: pos,
                positions,
            });
        }
    }

    fn any_at_position(&self, pos: &(i32, i32)) -> bool {
        self.tachyons.iter().any(|t| t.positions.contains(pos))
    }

    fn advance(&mut self, board: &HashMap<(i32, i32), Pos>) -> usize {
        let mut splits = 0;
        let mut actions = vec![];
        for (n, tachyon) in self
            .tachyons
            .iter()
            .enumerate()
            .filter(|(_, v)| v.is_active)
        {
            let (tcol, trow) = tachyon.cur_pos;
            let new_pos @ (new_tcol, new_trow) = (tcol, trow + 1);

            if let Some(v) = board.get(&new_pos) {
                match v {
                    Pos::Start => unreachable!(),
                    Pos::Empty => {
                        actions.push((n, TachyonAction::MoveTo(new_tcol, new_trow)));
                    }
                    Pos::Splitter => {
                        actions.push((n, TachyonAction::SplitAt(new_tcol, new_trow)));
                        splits += 1;
                    }
                }
            } else {
                // out of bounds
                actions.push((n, TachyonAction::OutOfBounds));
            }
        }

        for (n, action) in actions {
            match action {
                TachyonAction::OutOfBounds => self.tachyons[n].is_active = false,
                TachyonAction::SplitAt(col, row) => {
                    self.tachyons[n].is_active = false;
                    self.new((col - 1, row));
                    self.new((col + 1, row));
                }
                TachyonAction::MoveTo(col, row) => {
                    if self.any_at_position(&(col, row)) {
                        // kill/merge this tachyon
                        self.tachyons[n].is_active = false;
                    } else {
                        self.tachyons[n].cur_pos = (col, row);
                        self.tachyons[n].positions.insert((col, row));
                    }
                }
            }
        }

        splits
    }

    fn any_active(&self) -> bool {
        self.tachyons.iter().any(|t| t.is_active)
    }
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;

    debug_println!("{data:#?}");

    let mut tachyons = Tachyons { tachyons: vec![] };
    let (start_col, start_row) = data.start_pos();
    tachyons.new((start_col, start_row + 1));

    let mut total_splits = 0;
    loop {
        let splits = tachyons.advance(&data.board);

        total_splits += splits;

        if !tachyons.any_active() {
            break;
        }
    }

    println!("{total_splits}");

    Ok(())
}
