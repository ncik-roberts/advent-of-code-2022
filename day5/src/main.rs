struct Move {
    from: usize,
    to: usize,
    how_many: usize,
}

#[derive(Debug)]
struct Board {
    cols: Vec<Vec<char>>,
}

fn handle_move_9000(move_: Move, board: &mut Board) {
    for _ in 0..move_.how_many {
        let from_col = &mut board.cols[move_.from - 1];
        let moved = from_col.pop().unwrap();
        let to_col = &mut board.cols[move_.to - 1];
        to_col.push(moved);
    }
}

fn handle_move_9001(move_: Move, board: &mut Board) {
    for i in 0..move_.how_many {
        let from_col = &mut board.cols[move_.from - 1];
        let n = from_col.len();
        let moved = from_col.remove(n + i - move_.how_many);
        let to_col = &mut board.cols[move_.to - 1];
        to_col.push(moved);
    }
}

fn parse_board(lines: &mut advent::LineStream) -> Board {
    let mut reversed_cols: Vec<Vec<char>> = Vec::new();
    for _ in 0..100 {
        reversed_cols.push(Vec::new());
    }
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        }
        // 4 = "[T] ".len()
        for (i, chunk) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            match chunk[0] {
                ' ' => (),
                _ => {
                    let c = chunk[1];
                    reversed_cols[i].push(c);
                }
            }
        }
    }
    for col in &mut reversed_cols {
        col.reverse()
    }
    let cols = reversed_cols;
    Board { cols }
}

fn parse_moves(lines: &mut advent::LineStream) -> Vec<Move> {
    let mut moves = Vec::new();
    for line in lines {
        let tokens: Vec<&str> = line.split(" ").collect();
        let how_many: usize = tokens[1].parse().unwrap();
        let from: usize = tokens[3].parse().unwrap();
        let to: usize = tokens[5].parse().unwrap();
        let move_ = Move { from, to, how_many };
        moves.push(move_)
    }
    moves
}

fn print_board(board: Board) {
    for col in board.cols {
        match col.last() {
            Some(last) => print!("{last}"),
            None => (),
        }
    }
    println!()
}

fn part1(lines: advent::LineStream) {
    let mut lines = lines.into_iter();
    let mut board = parse_board(&mut lines);
    let moves = parse_moves(&mut lines);
    for move_ in moves {
        handle_move_9000(move_, &mut board);
    }
    print_board(board)
}

fn part2(lines: advent::LineStream) {
    let mut lines = lines.into_iter();
    let mut board = parse_board(&mut lines);
    let moves = parse_moves(&mut lines);
    for move_ in moves {
        handle_move_9001(move_, &mut board);
    }
    print_board(board);
}

fn main() {
    advent::of_code(advent::PARSE_AS_STREAM, part1, part2)
}
