use std::{collections::HashSet, io, vec};

use anyhow::Error;
use debug_print::debug_println;
use itertools::Itertools;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    num = { ASCII_DIGIT+ }
    row = { num ~ "," ~ num ~ "," ~ num }
    data = { SOI ~ row ~ (WHITE_SPACE+ ~ row)* ~ WHITE_SPACE* ~ EOI}
"#]
struct PestParser;

#[derive(Debug)]
struct Data {
    rows: Vec<(i32, i32, i32)>,
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
                let z = pairs.next().unwrap().as_str().parse().unwrap();
                rows.push((x, y, z));
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(Data { rows })
}

fn find_n_in_clusters(n: usize, clusters: &Vec<HashSet<usize>>) -> usize {
    clusters
        .iter()
        .enumerate()
        .find_map(
            |(i, cluster)| {
                if cluster.contains(&n) { Some(i) } else { None }
            },
        )
        .unwrap()
}

fn join_clusters(i1: usize, i2: usize, clusters: &mut Vec<HashSet<usize>>) {
    if i1 != i2 {
        let bigger = i1.max(i2);
        let smaller = i1.min(i2);
        let cluster1 = clusters.swap_remove(bigger);
        let cluster2 = clusters.swap_remove(smaller);
        let new_set = cluster1.union(&cluster2).cloned().collect();
        clusters.push(new_set);
    }
}

const TOP_N: usize = 1000;

pub fn main() -> Result<(), Error> {
    let data = parse()?;

    debug_println!("{data:#?}");

    let closest = data
        .rows
        .iter()
        .enumerate()
        .combinations(2)
        .map(|combination| {
            let (n1, (x1, y1, z1)) = combination[0];
            let (n2, (x2, y2, z2)) = combination[1];

            let dist = (x1 - x2) as i64 * (x1 - x2) as i64
                + (y1 - y2) as i64 * (y1 - y2) as i64
                + (z1 - z2) as i64 * (z1 - z2) as i64;

            (n1, n2, dist)
        })
        .sorted_by_key(|(_, _, dist)| *dist)
        .take(TOP_N)
        .collect_vec();

    let mut clusters = (0..data.rows.len())
        .map(|n| {
            let mut hs = HashSet::new();
            hs.insert(n);
            hs
        })
        .collect_vec();

    debug_println!("{clusters:#?}");

    for (n1, n2, _) in closest {
        let i1 = find_n_in_clusters(n1, &clusters);
        let i2 = find_n_in_clusters(n2, &clusters);
        join_clusters(i1, i2, &mut clusters);

        debug_println!("{clusters:#?}");
    }

    let result = clusters
        .iter()
        .map(|n| n.len())
        .sorted()
        .rev()
        .take(3)
        .fold(1, |acc, n| acc * n);

    println!("{result}");

    Ok(())
}
