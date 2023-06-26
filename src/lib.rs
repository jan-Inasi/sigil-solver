pub mod board;

use board::{Sigil, TetrisBlock, TetrisShape, TETRIS_SHAPE_COUNT};
use num_traits::FromPrimitive;

pub struct TetrisSet {
    ptr: usize,
    block_count: [u8; TETRIS_SHAPE_COUNT],
}

impl TetrisSet {
    pub fn new() -> Self {
        TetrisSet {
            ptr: 0,
            block_count: [0; TETRIS_SHAPE_COUNT],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.block_count[self.ptr] == 0
    }

    pub fn from_count_array(array: [u8; TETRIS_SHAPE_COUNT]) -> Self {
        let mut tetris_set = TetrisSet {
            ptr: 0,
            block_count: array,
        };
        tetris_set.rotate_to_nonempty();
        tetris_set
    }

    pub fn add(&mut self, block_shape: TetrisShape, count: u8) {
        // TODO overflow check
        self.block_count[block_shape as usize] += count;
        self.rotate_to_nonempty();
    }

    pub fn push(&mut self, block_shape: TetrisShape) {
        self.add(block_shape, 1);
    }

    fn decrement_count_on_ptr(&mut self) {
        self.block_count[self.ptr] -= 1;
        self.rotate_to_nonempty();
    }

    pub fn pop(&mut self) -> Option<TetrisShape> {
        if self.block_count[self.ptr] == 0 {
            return None;
        }

        let block_id = self.ptr;
        self.decrement_count_on_ptr();

        FromPrimitive::from_usize(block_id)
    }

    pub fn total_block_count(&self) -> usize {
        self.block_count.iter().map(|&x| x as usize).sum()
    }

    pub fn distinct_shapes_count(&self) -> u8 {
        self.block_count.iter().filter(|&x| x > &0).count() as u8
    }

    fn rotate_to_nonempty(&mut self) {
        if self.block_count[self.ptr] == 0 {
            self.rotate();
        }
    }

    pub fn rotate(&mut self) {
        for _ in 0..TETRIS_SHAPE_COUNT {
            self.ptr = (self.ptr + 1) % TETRIS_SHAPE_COUNT;
            if self.block_count[self.ptr] > 0 {
                break;
            }
        }
    }
}

pub fn solve(board: &mut board::Board, blocks: &mut TetrisSet) -> bool {
    if blocks.is_empty() {
        return true;
    }

    if board.capacity() < blocks.total_block_count() {
        return false;
    }

    if let Some(pos) = find_anchore(board) {
        for _ in 0..blocks.distinct_shapes_count() {
            let shape = blocks.pop().unwrap(); // checked before that isn't empty
            let sigil = Sigil::new(pos, TetrisBlock::default(shape));
            if solve_from_anchore(sigil, board, blocks) {
                return true;
            }
            blocks.push(shape);
            blocks.rotate();
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

fn solve_from_anchore(sigil: Sigil, board: &mut board::Board, blocks: &mut TetrisSet) -> bool {
    // let stdin = std::io::stdin();
    // let mut buffer = String::new();
    let mut sigil = sigil;
    for _ in 0..sigil.n_states() {
        // println!("{sigil:?}");
        // _ = stdin.read_line(&mut buffer);

        match board.add_sigil(sigil) {
            Ok(()) => {
                // println!("{board}");
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
