use std::collections::HashMap;
use crate::common::file_to_lines;
use crate::tokens::{parse_token_value_before, Token};

#[derive(Debug)]
struct Mapper {
    name: String,
    map: HashMap<usize, usize>,
    decorated_mapper: Option<Box<Mapper>>,
}

impl Mapper {
    fn from_lines_and_next(name: String, lines: Vec<String>, next: Option<Box<Mapper>>) -> Self {
        let map: HashMap<usize, usize> = lines.iter().map(|line| {
            let x: Vec<usize> = line.split_whitespace().map(|data_point| data_point.parse::<usize>().expect("invalid input, map data point is not a number")).collect();
            if let [destination_start, source_start, length] = x[..] {
                let source_range = source_start..source_start + length;
                let destination_range = destination_start..destination_start + length;

                return source_range.zip(destination_range);
            } else {
                panic!("Invalid input, mapping row does not consist of three numbers");
            }
        }).flatten().collect();
        Mapper {
            name,
            map,
            decorated_mapper: next,
        }
    }

    pub fn find_corresponding_value(&self, key: usize) -> usize {
        let mapped_value = self.map.get(&key).unwrap_or(&key).clone();

        self.decorated_mapper.as_ref().map(|map| map.find_corresponding_value(mapped_value)).unwrap_or(mapped_value)
    }
}

pub fn solve_a(path: &str) -> usize {
    let lines = file_to_lines(path);

    let seed_line = lines.iter().take(1).last().expect("invalid input, does not have a single line");
    let seeds: Vec<usize> = seed_line.replace("seeds: ", "").split_whitespace().map(|seed| seed.parse::<usize>().expect("Invalid seed, should be number")).collect();


    let remaining: Vec<String> = lines.iter().skip(1).map(|line| line.clone()).collect();

    let mut maps = chunks_by(remaining, "".to_string());
    maps.reverse();
    let first_map: Box<Mapper> = maps.iter().fold(None, |previous, map| {
        if let [heading, data @ ..] = &map[..] {
            let name: Token<String> = parse_token_value_before(heading, "map", "", " ").expect("missing heading for map");
            return Some(Box::new(Mapper::from_lines_and_next(
                name.1,
                data.to_owned().iter().map(|line| String::from(line.as_str())).collect(),
                previous,
            )));
        }

        panic!("Invalid map, insufficient lines given");
    }).expect("Invalid input, should have at least one map");

    println!("{:?}", first_map);


    seeds.iter().map(|seed| first_map.find_corresponding_value(seed.clone())).min().expect("Invalid input, should have at least one seed")
}

fn chunks_by(vec: Vec<String>, delimiter: String) -> Vec<Vec<String>> {
    let mut result = vec![];
    let mut chunk = vec![];

    vec.iter().for_each(|item| {
        if item == &delimiter {
            if !chunk.is_empty() {
                result.push(chunk.clone());
                chunk = vec![];
            }

        } else {
            chunk.push(item.clone());
        }
    });

    if !chunk.is_empty() {
        result.push(chunk.clone());
    }

    result
}