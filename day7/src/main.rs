use std::collections::HashMap;
use std::iter::Peekable;

struct State {
    dir: Vec<String>,
    dir_sizes: HashMap<String, u64>,
}

fn new_state() -> State {
    State {
        dir: Vec::new(),
        dir_sizes: HashMap::new(),
    }
}

#[derive(Debug)]
enum LsLine {
    Dir(String),
    File(u64, String),
}

#[derive(Debug)]
enum Cd {
    Root,
    Up,
    Dir(String),
}

type Ls = Vec<LsLine>;

#[derive(Debug)]
enum Command {
    Ls(Ls),
    Cd(Cd),
}

fn parse_ls_line(line: &String) -> Option<LsLine> {
    match line.as_bytes()[0] as char {
        '$' => None,
        _ => {
            let tokens = Vec::from_iter(line.split(" "));
            match tokens[0] {
                "dir" => Some(LsLine::Dir(tokens[1].to_string())),
                _ => {
                    let size = tokens[0].parse().unwrap();
                    let filename = tokens[1].to_string();
                    Some(LsLine::File(size, filename))
                }
            }
        }
    }
}

fn parse_ls(iter: &mut Peekable<advent::LineStream>) -> Ls {
    let mut result = Vec::new();
    fn wrap_parse_ls_line(line: Option<&String>) -> Option<LsLine> {
        let line = line?;
        parse_ls_line(line)
    }
    while let Some(ls_line) = wrap_parse_ls_line(iter.peek()) {
        iter.next();
        result.push(ls_line);
    }
    result
}

fn parse_cd(tokens: Vec<&str>) -> Cd {
    match tokens[2] {
        ".." => Cd::Up,
        "/" => Cd::Root,
        _ => Cd::Dir(tokens[2].to_string()),
    }
}

fn parse_command(line: String, iter: &mut Peekable<advent::LineStream>) -> Command {
    let tokens = Vec::from_iter(line.split(" "));
    match tokens[1] {
        "cd" => Command::Cd(parse_cd(tokens)),
        "ls" => Command::Ls(parse_ls(iter)),
        unknown => panic!("{unknown}"),
    }
}

fn handle_cd(cd: Cd, state: &mut State) {
    match cd {
        Cd::Root => state.dir.clear(),
        Cd::Dir(dir) => state.dir.push(dir),
        Cd::Up => {
            state.dir.pop();
            ()
        }
    }
}

fn handle_ls(ls: Ls, state: &mut State) {
    for ls_line in ls {
        match ls_line {
            LsLine::Dir(_) => (),
            LsLine::File(size, _) => {
                let mut acc = Vec::new();
                // always add to root
                *state.dir_sizes.entry("/".to_string()).or_default() += size;
                for dir in state.dir.to_owned() {
                    acc.push(dir);
                    let key: String = acc.join("/").to_string();
                    *state.dir_sizes.entry(key).or_default() += size;
                }
            }
        }
    }
}

fn handle_command(command: Command, state: &mut State) {
    match command {
        Command::Cd(cd) => handle_cd(cd, state),
        Command::Ls(ls) => handle_ls(ls, state),
    }
}

fn produce_state(lines: advent::LineStream) -> State {
    let mut state = new_state();
    let mut lines = lines.into_iter().peekable();
    while let Some(line) = lines.next() {
        let command = parse_command(line, &mut lines);
        handle_command(command, &mut state);
    }
    state
}

fn part1(lines: advent::LineStream) {
    let state = produce_state(lines);
    let mut total = 0;
    for key in state.dir_sizes.keys() {
        let val: u64 = *state.dir_sizes.get(key).unwrap();
        if val <= 100_000 {
            total += val
        }
    }
    println!("{total}")
}

fn part2(_lines: advent::LineStream) {
    ()
}

fn main() {
    advent::of_code(advent::PARSE_AS_STREAM, part1, part2)
}
