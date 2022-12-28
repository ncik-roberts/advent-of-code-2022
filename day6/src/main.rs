use std::collections::hash_map::RandomState;
use std::collections::HashSet;

fn run(line: String, num_chars: usize) {
    for i in 0..line.len() - num_chars {
        let set: HashSet<char, RandomState> = HashSet::from_iter(line[i..i + num_chars].chars());
        if set.len() == num_chars {
            println!("{}", i + num_chars);
            return;
        }
    }
}

fn part1(line: String) {
    run(line, 4);
}

fn part2(line: String) {
    run(line, 14);
}

fn main() {
    advent::of_code(advent::PARSE_AS_SINGLE_LINE, part1, part2)
}
