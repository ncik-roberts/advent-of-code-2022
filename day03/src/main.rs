use std::collections::hash_map::RandomState;
use std::collections::HashSet;

fn slice_to_hash_set<'a>(slice: &'a str) -> HashSet<&'a u8, RandomState> {
    let slice = slice.as_bytes();
    HashSet::from_iter(slice.iter())
}

fn priority(elem: char) -> u32 {
    let elem_u32 = elem as u32;
    match elem {
        'a'..='z' => elem_u32 - 'a' as u32 + 1,
        'A'..='Z' => elem_u32 - 'A' as u32 + 27,
        _ => panic!(),
    }
}

fn error_item(line: String) -> char {
    let n = line.len();
    let bin_size = n / 2;
    let set1 = slice_to_hash_set(&line[0..bin_size]);
    let set2 = slice_to_hash_set(&line[bin_size..n]);
    let mut intersection = set1.intersection(&set2);
    **intersection.next().unwrap() as char
}

fn badge(line1: String, line2: String, line3: String) -> char {
    let set1 = slice_to_hash_set(&line1);
    let set2 = slice_to_hash_set(&line2);
    let set3 = slice_to_hash_set(&line3);
    let intersection1 = HashSet::from_iter(set1.intersection(&set2).map(|u| *u));
    let mut intersection2 = set3.intersection(&intersection1);
    **intersection2.next().unwrap() as char
}

fn part1(lines: advent::LineStream) {
    let mut sum = 0;
    for line in lines {
        let error_item = error_item(line);
        sum += priority(error_item);
    }
    println!("{sum}")
}

fn part2(lines: advent::LineStream) {
    let mut sum = 0;
    let mut lines = lines.into_iter();
    while let Some(first) = lines.next() {
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();
        let badge = badge(first, second, third);
        sum += priority(badge);
    }
    println!("{sum}")
}

fn main() {
    advent::of_code(advent::PARSE_AS_STREAM, part1, part2)
}
