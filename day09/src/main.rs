use std::collections::HashSet;

enum Dir {
    D,
    R,
    U,
    L,
}

impl Dir {
    pub fn parse(str: &str) -> Dir {
        match str {
            "D" => Dir::D,
            "R" => Dir::R,
            "U" => Dir::U,
            "L" => Dir::L,
            _ => panic!(),
        }
    }
}

struct Move {
    dir: Dir,
    qty: u32,
}

impl Move {
    pub fn parse(str: String) -> Move {
        let sections = Vec::from_iter(str.split(' '));
        Move {
            dir: Dir::parse(sections[0]),
            qty: sections[1].parse().unwrap(),
        }
    }
}

fn diff_row_and_col(x: (i32, i32), y: (i32, i32)) -> bool {
    x.0 != y.0 && x.1 != y.1
}

fn fix_follower(leader: (i32, i32), follower: (i32, i32), old_leader: (i32, i32)) -> (i32, i32) {
    if !diff_row_and_col(leader, follower) {
        ((leader.0 + follower.0) / 2, (leader.1 + follower.1) / 2)
    } else if diff_row_and_col(follower, old_leader) {
        old_leader
    } else {
        let mut result = old_leader;
        if (leader.0 - follower.0).abs() == 1 {
            result.0 = leader.0
        } else {
            result.1 = leader.1
        }
        result
    }
}

fn run(lines: advent::LineStream, n: u32) {
    // first elem is the head
    let first_elem = (0, 0);
    let mut positions = Vec::new();
    for _ in 0..n + 1 {
        positions.push(first_elem)
    }
    let mut seen_tails: HashSet<(i32, i32)> = HashSet::new();
    seen_tails.insert(first_elem);
    for mov in lines.map(Move::parse) {
        for _ in 0..mov.qty {
            let mut old_leader = positions[0];
            let mut head = positions[0];
            match mov.dir {
                Dir::U => head.0 -= 1,
                Dir::D => head.0 += 1,
                Dir::R => head.1 += 1,
                Dir::L => head.1 -= 1,
            }
            positions[0] = head;
            for i in 1..positions.len() {
                let leader = positions[i - 1];
                let follower = positions[i];
                if (follower.0 - leader.0).abs() > 1 || (follower.1 - leader.1).abs() > 1 {
                    positions[i] = fix_follower(leader, follower, old_leader);
                }
                old_leader = follower;
            }
            seen_tails.insert(*positions.last().unwrap());
        }
    }
    let count = seen_tails.len();
    println!("{count}")
}

fn part1(lines: advent::LineStream) {
    run(lines, 1)
}

fn part2(lines: advent::LineStream) {
    run(lines, 9)
}

fn main() {
    advent::of_code(advent::PARSE_AS_STREAM, part1, part2)
}
