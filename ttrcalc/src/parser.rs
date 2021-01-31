use std::env;
use std::fs;

use crate::game::Game;
use crate::route::Route;

fn parse_csv_line(input: &str) -> Vec<&str> {
    println!("Parse {}", input);
    input.split(",").collect()
}

fn parse_csv(input_content: &str) -> Vec<Vec<&str>> {
    input_content
        .split("\r\n")
        .map(|line| parse_csv_line(line))
        .collect()
}

fn parse_csv_file<'a>(fname: &str, buffer: &'a mut String) -> Vec<Vec<&'a str>> {
    let result: String =
        fs::read_to_string(fname).expect(format!("File {} not found!", fname).as_str());
    buffer.clone_from(&result);
    let parsed = parse_csv(buffer.as_str());
    return parsed;
}

pub fn parse_routes(fname: &str, game: &mut Game) {
    let mut buffer = String::new();
    let csv = parse_csv_file(fname, &mut buffer);
    let csv = csv.iter().skip(1); // skip header
    for line in csv {
        if line.len() < 2 {
            continue;
        }
        let city1_id = game.add_get_city(String::from(line[0]));
        let city2_id = game.add_get_city(String::from(line[1]));
        let length = line[2].parse::<usize>();
        println!(
            "{:?}={},{:?}={},{:?} ==> {:?}",
            line[0], city1_id, line[1], city2_id, line[2], length
        );
        let length = length.expect(format!("Cannot parse number \"{}\"", line[2]).as_str());
        let route = Route::new(
            city1_id,
            city2_id,
            crate::route::unknown_track_of_length(length),
        );
        game.add_route(route);
    }
}
