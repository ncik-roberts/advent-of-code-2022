#[derive(PartialEq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

enum WhatHappens {
    Win,
    Lose,
    Draw,
}

fn rps_of_first(c: char) -> Rps {
    match c {
        'A' => Rps::Rock,
        'B' => Rps::Paper,
        'C' => Rps::Scissors,
        _ => panic!(),
    }
}

fn rps_of_second(c: char) -> Rps {
    match c {
        'X' => Rps::Rock,
        'Y' => Rps::Paper,
        'Z' => Rps::Scissors,
        _ => panic!(),
    }
}

fn what_happens_of_second(c: char) -> WhatHappens {
    match c {
        'X' => WhatHappens::Lose,
        'Y' => WhatHappens::Draw,
        'Z' => WhatHappens::Win,
        _ => panic!(),
    }
}

fn what_happens_score(what: &WhatHappens) -> u32 {
    match what {
        WhatHappens::Win => 6,
        WhatHappens::Lose => 0,
        WhatHappens::Draw => 3,
    }
}

fn beats(x: Rps) -> Rps {
    match x {
        Rps::Rock => Rps::Scissors,
        Rps::Paper => Rps::Rock,
        Rps::Scissors => Rps::Paper,
    }
}

fn beaten_by(x: Rps) -> Rps {
    match x {
        Rps::Rock => Rps::Paper,
        Rps::Paper => Rps::Scissors,
        Rps::Scissors => Rps::Rock,
    }
}

fn what_i_play(theirs: Rps, what: &WhatHappens) -> Rps {
    match what {
        WhatHappens::Win => beaten_by(theirs),
        WhatHappens::Lose => beats(theirs),
        WhatHappens::Draw => theirs,
    }
}

fn my_score_of_what_happens(theirs: &Rps, mine: &Rps) -> u32 {
    if theirs == mine {
        3
    } else {
        match (theirs, mine) {
            (Rps::Paper, Rps::Scissors) => 6,
            (Rps::Scissors, Rps::Rock) => 6,
            (Rps::Rock, Rps::Paper) => 6,
            _ => 0,
        }
    }
}

fn shape_score(mine: &Rps) -> u32 {
    match mine {
        Rps::Rock => 1,
        Rps::Paper => 2,
        Rps::Scissors => 3,
    }
}

fn part1(lines: advent::LineStream) {
    let mut total = 0;
    for line in lines {
        let line = line.as_bytes();
        let theirs = rps_of_first(char::from(line[0]));
        let mine = rps_of_second(char::from(line[2]));
        total += shape_score(&mine) + my_score_of_what_happens(&theirs, &mine);
    }
    println!("{total}");
}

fn part2(lines: advent::LineStream) {
    let mut total = 0;
    for line in lines {
        let line = line.as_bytes();
        let theirs = rps_of_first(char::from(line[0]));
        let what = &what_happens_of_second(char::from(line[2]));
        let mine = what_i_play(theirs, what);
        total += shape_score(&mine) + what_happens_score(what);
    }
    println!("{total}");
}

fn main() {
    advent::of_code(advent::PARSE_AS_STREAM, part1, part2);
}
