#[derive(Debug)]
struct Board {
    rows: Vec<Vec<u8>>,
}

impl Board {
    pub fn new(lines: advent::LineStream) -> Board {
        Board {
            rows: Vec::from_iter(lines.map(|s| s.as_bytes().to_vec())),
        }
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn width(&self) -> usize {
        self.rows[0].len()
    }

    // equal size board full of zeroes
    pub fn constant(&self, val: u32) -> Vec<Vec<u32>> {
        Vec::from_iter(
            self.rows
                .clone()
                .into_iter()
                .map(|v| Vec::from_iter(v.into_iter().map(|_| val))),
        )
    }
}

struct BoardIterator<'a> {
    board: &'a Board,
    row: isize,
    col: isize,
    row_incr: isize,
    col_incr: isize,
}

struct BoardIteratorItem {
    row: usize,
    col: usize,
    elem: u8,
}

impl BoardIterator<'_> {
    pub fn left<'a>(board: &'a Board, row: usize) -> BoardIterator<'a> {
        BoardIterator {
            board,
            row: row as isize,
            col: (board.rows[0].len() - 1) as isize,
            row_incr: 0,
            col_incr: -1,
        }
    }

    pub fn right<'a>(board: &'a Board, row: usize) -> BoardIterator<'a> {
        BoardIterator {
            board,
            row: row as isize,
            col: 0,
            row_incr: 0,
            col_incr: 1,
        }
    }

    pub fn up<'a>(board: &'a Board, col: usize) -> BoardIterator<'a> {
        BoardIterator {
            board,
            row: (board.rows.len() - 1) as isize,
            col: col as isize,
            row_incr: -1,
            col_incr: 0,
        }
    }

    pub fn down<'a>(board: &'a Board, col: usize) -> BoardIterator<'a> {
        BoardIterator {
            board,
            row: 0,
            col: col as isize,
            row_incr: 1,
            col_incr: 0,
        }
    }
}

impl Iterator for BoardIterator<'_> {
    type Item = BoardIteratorItem;
    fn next(&mut self) -> Option<BoardIteratorItem> {
        let row = self.row;
        let col = self.col;
        if row >= 0
            && row < self.board.rows.len() as isize
            && col >= 0
            && col < self.board.rows[0].len() as isize
        {
            let elem = self.board.rows[row as usize][col as usize];
            self.row += self.row_incr;
            self.col += self.col_incr;
            Some(BoardIteratorItem {
                row: row as usize,
                col: col as usize,
                elem,
            })
        } else {
            None
        }
    }
}

fn count(counts: &mut Vec<Vec<u32>>, iterator: BoardIterator) {
    let mut running_highest: i16 = -1;
    for item in iterator {
        let elem: i16 = item.elem as i16;
        if elem <= running_highest {
            counts[item.row][item.col] += 1
        }
        running_highest = running_highest.max(elem)
    }
}

fn add_score(scores: &mut Vec<Vec<u32>>, iterator: BoardIterator) {
    let mut seen: Vec<u8> = Vec::new();
    for item in iterator {
        let mut score = 0;
        for seen_item in &mut seen.iter().rev() {
            score += 1;
            if *seen_item >= item.elem {
                break;
            }
        }
        scores[item.row][item.col] *= score;
        seen.push(item.elem);
    }
}

fn part1(lines: advent::LineStream) {
    let board = Board::new(lines);
    let mut counts = board.constant(0);
    for row in 0..board.height() {
        count(&mut counts, BoardIterator::left(&board, row));
        count(&mut counts, BoardIterator::right(&board, row));
    }
    for col in 0..board.width() {
        count(&mut counts, BoardIterator::down(&board, col));
        count(&mut counts, BoardIterator::up(&board, col));
    }

    let mut count_visible = 0;
    for count_row in counts {
        for count in count_row {
            if count != 4 {
                count_visible += 1
            }
        }
    }

    println!("{count_visible}")
}

fn part2(lines: advent::LineStream) {
    let board = Board::new(lines);
    let mut scores = board.constant(1);
    for row in 0..board.height() {
        add_score(&mut scores, BoardIterator::left(&board, row));
        add_score(&mut scores, BoardIterator::right(&board, row));
    }
    for col in 0..board.width() {
        add_score(&mut scores, BoardIterator::down(&board, col));
        add_score(&mut scores, BoardIterator::up(&board, col));
    }

    let best_score = scores
        .iter()
        .fold(0, |acc, row| acc.max(*row.iter().max().unwrap()));

    println!("{best_score}")
}

fn main() {
    advent::of_code(advent::PARSE_AS_STREAM, part1, part2)
}
