use std::{collections::HashSet, io, vec};

use anyhow::Error;
use debug_print::debug_println;
use itertools::Itertools;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    num = { ASCII_DIGIT+ }
    row = { num ~ "," ~ num }
    data = { SOI ~ row ~ (WHITE_SPACE+ ~ row)* ~ WHITE_SPACE* ~ EOI}
"#]
struct PestParser;

#[derive(Debug)]
struct Data {
    rows: Vec<(i32, i32)>,
}

fn parse() -> Result<Data, Error> {
    let mut rows = vec![];

    let input = io::read_to_string(io::stdin())?;
    let mut data = PestParser::parse(Rule::data, &input)?;
    for pair in data.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::row => {
                let mut pairs = pair.into_inner();
                let x = pairs.next().unwrap().as_str().parse().unwrap();
                let y = pairs.next().unwrap().as_str().parse().unwrap();
                rows.push((x, y));
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(Data { rows })
}

fn calc_area((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> i64 {
    ((x1 - x2).abs() + 1) as i64 * ((y1 - y2).abs() + 1) as i64
}

fn point_in_area((px, py): &(i32, i32), (x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> bool {
    px > x1.min(x2) && px < x1.max(x2) && py > y1.min(y2) && py < y1.max(y2)
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    p1: (i32, i32),
    p2: (i32, i32),
}

#[derive(Debug)]
struct Tiles {
    red_tiles: HashSet<(i32, i32)>,
    perimeter_tiles: HashSet<(i32, i32)>,
}

fn make_lines(data: &Data) -> Vec<Line> {
    let init = data.rows[0];
    let mut lines = data
        .rows
        .iter()
        .skip(1)
        .fold((vec![], init), |(mut acc, prev), x| {
            acc.push(Line {
                p1: prev,
                p2: x.clone(),
            });
            (acc, x.clone())
        })
        .0;

    lines.push(Line {
        p1: lines.last().unwrap().p2,
        p2: lines.first().unwrap().p1,
    });
    lines
}

fn build_tiles(lines: &Vec<Line>) -> Tiles {
    let mut red_tiles = HashSet::new();
    let mut perimeter_tiles = HashSet::new();

    lines.iter().for_each(|line| {
        red_tiles.insert(line.p1);

        let p1 = line.p1;
        let p2 = line.p2;

        if p1.0 == p2.0 {
            let min = p1.1.min(p2.1);
            let max = p1.1.max(p2.1);

            for y in min..=max {
                perimeter_tiles.insert((p1.0, y));
            }
        } else {
            let min = p1.0.min(p2.0);
            let max = p1.0.max(p2.0);

            for x in min..=max {
                perimeter_tiles.insert((x, p1.1));
            }
        }
    });

    Tiles {
        red_tiles,
        perimeter_tiles,
    }
}

pub fn main() -> Result<(), Error> {
    let data = parse()?;

    debug_println!("{data:#?}");

    let lines = make_lines(&data);

    debug_println!("{lines:#?}");

    let tiles = build_tiles(&lines);

    let areas = data
        .rows
        .iter()
        .combinations(2)
        .map(|v| (v[0], v[1], calc_area(v[0], v[1])))
        .sorted_by_key(|(_, _, area)| *area)
        .rev()
        .collect_vec();

    let mut result = 0;
    for (p1, p2, area) in areas {
        let mut invalid = false;

        // check reds
        for red_tile in tiles.red_tiles.iter() {
            if point_in_area(red_tile, p1, p2) {
                invalid = true;
                break;
            }
        }

        // check perimeters
        for perimeter_tile in tiles.perimeter_tiles.iter() {
            if point_in_area(perimeter_tile, p1, p2) {
                invalid = true;
                break;
            }
        }

        if !invalid {
            result = area;
            break;
        }
    }

    println!("{}", result);
    Ok(())
}
