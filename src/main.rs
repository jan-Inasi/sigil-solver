mod sigils;

use sigils::board::{Board, Direction, Line, Tetris};

fn main() {
    let mut board = Board::new(6, 6);
    let mut blocks = Vec::new();
    blocks.extend([Tetris::Square; 1].into_iter());
    blocks.extend([Tetris::Line(Line::Vertical); 0].into_iter());
    blocks.extend([Tetris::S(Line::Horizontal); 4].into_iter());
    blocks.extend([Tetris::Z(Line::Vertical); 0].into_iter());
    blocks.extend([Tetris::L(Direction::Down); 4].into_iter());
    blocks.extend([Tetris::J(Direction::Down); 0].into_iter());
    blocks.extend([Tetris::T(Direction::Down); 0].into_iter());

    println!("{}", sigils::solve(&mut board, &mut blocks));

    println!("{board}");
}
