use std::iter::Successors;

fn powers_of_5() -> Successors<i64, fn(&i64) -> Option<i64>> {
    std::iter::successors(Some(1i64), |n| n.checked_mul(5))
}

fn of_snafu(line: String) -> i64 {
    line.chars()
        .rev()
        .map(|c| match c {
            '1' => 1,
            '2' => 2,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        })
        .zip(powers_of_5())
        .map(|(x, y)| x * y)
        .sum()
}

fn snafu_adjustment(num: i64) -> i64 {
    let scanned = powers_of_5().map(|x| x * 2).scan(0, |state, n| {
        let result = *state + n;
        *state = result;
        Some(result)
    });

    let mut prev: i64 = 0;
    for max_representable_with_num_digits in scanned {
        println!("max: {max_representable_with_num_digits}");
        if num <= max_representable_with_num_digits {
            return prev;
        }
        prev = max_representable_with_num_digits
    }
    panic!()
}

fn to_base5(num: i64) -> Vec<i64> {
    let mut start = 1;
    let mut remaining = num;
    while start <= num {
        start *= 5
    }
    start /= 5;
    let mut digits = Vec::new();
    while start > 0 {
        let digit = remaining / start;
        digits.push(digit);
        remaining %= start;
        start /= 5;
    }
    digits
}

fn deadjust(x: i64) -> &'static str {
    match x {
        0 => "=",
        1 => "-",
        2 => "0",
        3 => "1",
        4 => "2",
        _ => panic!(),
    }
}

fn to_snafu(num: i64) -> String {
    let adjustment: i64 = snafu_adjustment(num);
    let adjusted_base5 = to_base5(num + adjustment);
    let mut result: String = adjusted_base5.first().unwrap().to_string();
    for c in adjusted_base5[1..].iter() {
        let str = deadjust(*c).to_string();
        result.push_str(&str[..])
    }
    result
}

fn part1(lines: advent::LineStream) {
    let sum: i64 = lines.map(of_snafu).sum();
    let snafued = to_snafu(sum);
    println!("{snafued}")
}

fn part2(_lines: advent::LineStream) {
    ()
}

fn main() {
    advent::of_code(advent::PARSE_AS_STREAM, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_digits() {
        assert_eq!(snafu_adjustment(1), 0);
        assert_eq!(snafu_adjustment(2), 0);
        assert_eq!(snafu_adjustment(3), 2);
        assert_eq!(snafu_adjustment(4), 2);
        assert_eq!(snafu_adjustment(5), 2);
        assert_eq!(snafu_adjustment(12), 2);
        assert_eq!(snafu_adjustment(13), 12);
    }

    #[test]
    fn test_base5() {
        assert_eq!(to_base5(1), vec![1]);
        assert_eq!(to_base5(2), vec![2]);
        assert_eq!(to_base5(3), vec![3]);
        assert_eq!(to_base5(5), vec![1, 0]);
        assert_eq!(to_base5(24), vec![4, 4]);
        assert_eq!(to_base5(25), vec![1, 0, 0]);
        assert_eq!(to_base5(26), vec![1, 0, 1]);
        assert_eq!(to_base5(30), vec![1, 1, 0]);
    }

    #[test]
    fn more_test1() {
        assert_eq!(to_snafu(1), "1");
        assert_eq!(to_snafu(2), "2");
        assert_eq!(to_snafu(3), "1=");
        assert_eq!(to_snafu(4), "1-");
        assert_eq!(to_snafu(5), "10");
        assert_eq!(to_snafu(6), "11");
        assert_eq!(to_snafu(12), "22");
        assert_eq!(to_snafu(13), "1==");
    }

    #[test]
    fn more_test2() {
        assert_eq!(of_snafu("1=0-2-1-0=20-01-2-21".to_string()), 11347396408986);
        assert_eq!(to_snafu(11347396408986), "1=0-2-1-0=20-01-2-21");
    }
}
