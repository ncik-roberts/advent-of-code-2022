use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::Lines;
use std::io::{self, BufRead};
use std::iter::Map;

pub struct HowToParse<T> {
    how_to_parse: fn(&Vec<String>) -> T,
}

pub fn invalid_args<Any>(args: &Vec<String>) -> Any {
    let args = args.clone();
    panic!("invalid args: {args:?}")
}

pub type LineStream = Map<Lines<BufReader<File>>, fn(Result<String, Error>) -> String>;

fn parse_as_stream(args: &Vec<String>) -> LineStream {
    let filename = args.first().unwrap_or_else(|| invalid_args(args));
    let file = File::open(&filename);
    let file = file.unwrap_or_else(|err| panic!("Invalid file: {filename}. {err}"));
    io::BufReader::new(file).lines().map(|l| l.unwrap())
}

pub const PARSE_AS_STREAM: HowToParse<LineStream> = HowToParse {
    how_to_parse: parse_as_stream,
};

pub fn of_code<T>(how_to_parse: HowToParse<T>, part1: fn(T) -> (), part2: fn(T) -> ()) -> () {
    let args: Vec<String> = env::args().collect();
    let mut working_args = args.clone();
    working_args.reverse();
    working_args.pop();
    let cmd = working_args.pop().unwrap_or_else(|| invalid_args(&args));
    let t = (how_to_parse.how_to_parse)(&working_args);
    match &cmd[..] {
        "part1" => part1(t),
        "part2" => part2(t),
        "both" => {
            print!("Part 1: ");
            part1(t);
            print!("Part 2: ");
            let reparsed_t = (how_to_parse.how_to_parse)(&working_args);
            part2(reparsed_t);
        }
        _ => invalid_args(&args),
    }
}
