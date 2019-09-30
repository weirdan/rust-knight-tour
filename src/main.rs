type Point = (i32, i32);

#[derive(Debug, Clone)]
struct Board {
    cells: Vec<Vec<bool>>,
}

impl Board {
    fn new(size: usize) -> Board {
        Board {
            cells: vec![vec![false; size]; size],
        }
    }
    fn visit(&mut self, point: &Point) {
        self.cells[point.0 as usize][point.1 as usize] = true;
    }
    fn show_path(&self, path: &Vec<Point>) {
        let mut i = 0;
        let mut shown_board = vec![vec![String::from("_"); self.size()]; self.size()];
        for point in path {
            i += 1;
            shown_board[point.0 as usize][point.1 as usize] = i.to_string();
        }
        for row in shown_board {
            println!("{:?}", row);
        }
    }
    fn size(&self) -> usize {
        self.cells.len()
    }
    fn visited(&self, point: &Point) -> bool {
        self.cells[point.0 as usize][point.1 as usize]
    }
}

fn main() {
    let mut board = Board::new(6);
    let initial_position = (0, 0);
    board.visit(&initial_position);
    run(&board, &vec![initial_position]);
}

fn run(board: &Board, path: &Vec<Point>) {
    if path.len() >= board.size() * board.size() {
        println!("Found solution");
        board.show_path(&path);
        std::process::exit(0);
    }
    for one_move in allowed_moves(board, path.last().expect("empty path")) {
        let mut new_path = path.clone();
        let mut new_board = board.clone();
        new_path.push(one_move);
        new_board.visit(&one_move);
        run(&new_board, &new_path);
    }
    if path.len() == 1 {
        println!("No solutions");
        std::process::exit(1);
    }
}

fn knight_moves(from: &Point) -> Vec<Point> {
    vec![
        (from.0 + 1, from.1 + 2),
        (from.0 + 2, from.1 + 1),
        (from.0 + 2, from.1 - 1),
        (from.0 + 1, from.1 - 2),
        (from.0 - 1, from.1 - 2),
        (from.0 - 2, from.1 - 1),
        (from.0 - 2, from.1 + 1),
        (from.0 - 1, from.1 + 2),
    ]
}

fn allowed_moves(b: &Board, from: &Point) -> Vec<Point> {
    knight_moves(from)
        .into_iter()
        .filter(|next| {
            next.0 >= 0
                && next.0 < b.size() as i32
                && next.1 >= 0
                && next.1 < b.size() as i32
                && !b.visited(next)
        })
        .collect::<Vec<Point>>()
}
