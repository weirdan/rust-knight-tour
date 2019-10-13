use std::time::Instant;
use std::thread;

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
    #[allow(dead_code)]
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
    let size = 16;

    let total = Instant::now();
    let mut threads = Vec::new();
    for i in 0..size {
        for j in 0..size {
            threads.push(thread::spawn(move || solve(size, i, j)))
        }
    }
    for t in threads {
        t.join().unwrap()
    }
    println!("Total time: {:?}", total.elapsed());
}

fn solve(size: usize, i: usize, j: usize) {
    let mut board = Board::new(size);
    let initial_position = (i as i32, j as i32);
    board.visit(&initial_position);
    let start = Instant::now();
    match run(&board, &vec![initial_position]) {
        Ok(_) => println!("{:?} {:?}", initial_position, start.elapsed()),
        Err(_) => println!("{:?} No solutions, {:?}", initial_position, start.elapsed()),
    }
}

fn run(board: &Board, path: &Vec<Point>) -> Result<Vec<Point>, bool> {
    if path.len() >= board.size() * board.size() {
        return Ok(path.clone());
    }
    let mut sorted_moves = allowed_moves(board, path.last().unwrap());
    sorted_moves.sort_by(|a, b| {
        allowed_moves(board, a)
            .len()
            .cmp(&allowed_moves(board, b).len())
    });
    for one_move in sorted_moves {
        let mut new_path = path.clone();
        let mut new_board = board.clone();
        new_path.push(one_move);
        new_board.visit(&one_move);
        match run(&new_board, &new_path) {
            Ok(path) => return Ok(path),
            Err(_) => (),
        }
    }
    Err(true)
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

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn knight_has_8_moves() {
        let all_moves = knight_moves(&(10, 10));
        assert_eq!(8, all_moves.len());
    }

    #[test]
    fn knight_in_top_left_corner_has_2_moves() {
        let top_left_corner_moves = allowed_moves(&Board::new(5), &(0, 0));
        assert_eq!(2, top_left_corner_moves.len());
    }

    #[test]
    fn knight_ignores_moves_to_visited_squares() {
        let mut board = Board::new(4);
        board.visit(&(0, 0));
        board.visit(&(2, 1));
        board.visit(&(1, 2));
        let moves = allowed_moves(&board, &(0, 0));
        assert_eq!(0, moves.len());
    }

    #[test]
    fn knight_cannot_tour_on_3x3_board() {
        for i in 0..3 {
            for j in 0..3 {
                let mut board = Board::new(3);
                let initial_position = (i as i32, j as i32);
                board.visit(&initial_position);
                run(&board, &vec![initial_position]).expect_err("Should have failed");
            }
        }
    }

    #[test]
    fn knight_can_tour_on_6x6_board() {
        for i in 0..6 {
            for j in 0..6 {
                let mut board = Board::new(6);
                let initial_position = (i as i32, j as i32);
                board.visit(&initial_position);
                run(&board, &vec![initial_position]).expect("Should have passed");
            }
        }
    }
}
