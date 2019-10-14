use std::thread;
use std::time::Instant;

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
    fn visit(&mut self, &(x, y): &Point) {
        self.cells[x as usize][y as usize] = true;
    }
    #[allow(dead_code)]
    fn show_path(&self, path: &Vec<Point>) {
        let mut shown_board = vec![vec![String::from("_"); self.size()]; self.size()];
        for (i, &(x, y)) in path.iter().enumerate() {
            shown_board[x as usize][y as usize] = i.to_string();
        }
        for row in shown_board {
            println!("{:?}", row);
        }
    }
    fn size(&self) -> usize {
        self.cells.len()
    }
    fn visited(&self, &(x, y): &Point) -> bool {
        self.cells[x as usize][y as usize]
    }
}

fn main() {
    let size = 16;

    let total = Instant::now();
    let mut threads = Vec::new();
    for i in 0..size {
        for j in 0..size {
            threads.push(thread::spawn(move || run(size, i, j)))
        }
    }
    for t in threads {
        t.join().unwrap()
    }
    println!("Total time: {:?}", total.elapsed());
}

fn run(size: usize, i: usize, j: usize) {
    let mut board = Board::new(size);
    let initial_position = (i as i32, j as i32);
    board.visit(&initial_position);
    let start = Instant::now();
    match solve(&board, &vec![initial_position]) {
        Some(_) => println!("{:?} {:?}", initial_position, start.elapsed()),
        None => println!("{:?} No solutions, {:?}", initial_position, start.elapsed()),
    }
}

fn solve(board: &Board, path: &Vec<Point>) -> Option<Vec<Point>> {
    if path.len() >= board.size() * board.size() {
        return Some(path.clone());
    }
    let mut moves = allowed_moves(&board, path.last().unwrap());
    // try branch having minumum of next moves first
    moves.sort_by(|a, b| {
        let next_a = allowed_moves(&board, a);
        let next_b = allowed_moves(&board, b);
        next_a.len().cmp(&next_b.len())
    });
    for one_move in moves {
        let mut new_path = path.clone();
        let mut new_board = board.clone();
        new_path.push(one_move);
        new_board.visit(&one_move);
        if let Some(path) = solve(&new_board, &new_path) {
            return Some(path);
        }
    }
    None
}

fn knight_moves(&(x, y): &Point) -> Vec<Point> {
    vec![
        (x + 1, y + 2),
        (x + 2, y + 1),
        (x + 2, y - 1),
        (x + 1, y - 2),
        (x - 1, y - 2),
        (x - 2, y - 1),
        (x - 2, y + 1),
        (x - 1, y + 2),
    ]
}

fn allowed_moves(b: &Board, from: &Point) -> Vec<Point> {
    let size = b.size() as i32;
    knight_moves(from)
        .into_iter()
        .filter(|&(x, y)| {
            x >= 0 && x < size 
            && y >= 0 && y < size 
            && !b.visited(&(x, y))
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
                if let Some(_) = solve(&board, &vec![initial_position]) {
                    panic!("Should have failed");
                }
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
                solve(&board, &vec![initial_position]).expect("Should have passed");
            }
        }
    }
}
