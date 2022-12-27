fn parse(token: &str) -> (u32, u32) {
    let tokens = token.split("-");
    let tokens = Vec::from_iter(tokens);
    let lo: u32 = tokens[0].parse().unwrap();
    let hi: u32 = tokens[1].parse().unwrap();
    (lo, hi)
}

fn run(lines: advent::LineStream, condition: fn(u32, u32, u32, u32) -> bool) -> () {
    let mut count = 0;
    for line in lines {
        let tokens = line.split(",");
        let tokens = Vec::from_iter(tokens);
        let (lo1, hi1) = parse(tokens[0]);
        let (lo2, hi2) = parse(tokens[1]);
        if condition(lo1, lo2, hi1, hi2) {
            count += 1
        }
    }
    println!("{count}")
}

fn part1(lines: advent::LineStream) {
    run(lines, |lo1, lo2, hi1, hi2| {
        lo1 <= lo2 && hi2 <= hi1 || lo2 <= lo1 && hi1 <= hi2
    })
}

fn part2(lines: advent::LineStream) {
    run(lines, |lo1, lo2, hi1, hi2| {
        lo1 <= lo2 && hi1 >= lo2 || lo2 <= lo1 && hi2 >= lo1
    })
}

fn main() {
    advent::of_code(advent::PARSE_AS_STREAM, part1, part2)
}
