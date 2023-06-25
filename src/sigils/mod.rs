pub mod board;

use board::{Sigil, Tetris};

pub fn solve(board: &mut board::Board, blocks: &mut Vec<Tetris>) -> bool {
    if blocks.is_empty() {
        return true;
    }

    if board.capacity() < blocks.len() {
        return false;
    }

    if let Some(pos) = find_anchore(board) {
        for _ in 0..blocks.len() {
            let block = blocks.pop().unwrap(); // checked before that isn't empty
            let sigil = Sigil::new(pos, block.clone());
            if solve_from_anchore(sigil, board, blocks) {
                return true;
            }
            blocks.push(block);
            blocks.rotate_right(1);
        }
    }
    return false;
}

fn find_anchore(board: &board::Board) -> Option<(u8, u8)> {
    for y in 0..board.height() {
        for x in 0..board.width() {
            if board.is_cell_avaiable((x as i8, y as i8)) {
                return Some((x, y));
            }
        }
    }
    None
}

fn solve_from_anchore(sigil: Sigil, board: &mut board::Board, blocks: &mut Vec<Tetris>) -> bool {
    let mut sigil = sigil;
    for _ in 0..sigil.n_states() {
        // println!("{sigil:?}");
        // println!("{board}");
        match board.add_sigil(sigil) {
            Ok(()) => {
                if solve(board, blocks) {
                    return true;
                }
                if let Some(s) = board.pop_sigil() {
                    sigil = s;
                } else {
                    return false;
                }
            }
            Err(sig) => sigil = sig,
        }
        sigil.rotate();
    }
    return false;
}
