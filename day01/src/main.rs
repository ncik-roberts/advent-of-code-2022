fn part1(lines: advent::LineStream) {
    let mut max = 0;
    let mut sum = 0;
    for line in lines {
        if line == "" {
            max = u32::max(max, sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    }
    max = u32::max(max, sum);
    println!("{max}");
}

fn part2(lines: advent::LineStream) {
    let mut all = vec![];
    let mut sum = 0;
    for line in lines {
        if line == "" {
            all.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    }
    all.push(sum);
    all.sort();
    all.reverse();
    let total = all[0] + all[1] + all[2];
    println!("{total}");
}

fn main() {
    advent::of_code(advent::PARSE_AS_STREAM, part1, part2);
}
