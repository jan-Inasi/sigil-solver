mod arg_parsing;
// pub mod sigils;

use sigils::board::{Board, Tetris};

fn main() {
    let cfg = arg_parsing::get_parsed_arguments_or_exit();

    let mut board = Board::new(cfg.width, cfg.height);
    let mut blocks = Vec::new();
    blocks.extend(vec![Tetris::o_default(); cfg.o_count].into_iter());
    blocks.extend(vec![Tetris::z_default(); cfg.z_count].into_iter());
    blocks.extend(vec![Tetris::t_default(); cfg.t_count].into_iter());
    blocks.extend(vec![Tetris::l_default(); cfg.l_count].into_iter());
    blocks.extend(vec![Tetris::j_default(); cfg.j_count].into_iter());
    blocks.extend(vec![Tetris::i_default(); cfg.i_count].into_iter());
    blocks.extend(vec![Tetris::s_default(); cfg.s_count].into_iter());

    // let mut board = Board::new(6, 6);
    // let mut blocks = Vec::new();
    // blocks.extend([Tetris::o_default(); 1].into_iter());
    // blocks.extend([Tetris::i_default(); 0].into_iter());
    // blocks.extend([Tetris::s_default(); 4].into_iter());
    // blocks.extend([Tetris::z_default(); 0].into_iter());
    // blocks.extend([Tetris::l_default(); 4].into_iter());
    // blocks.extend([Tetris::j_default(); 0].into_iter());
    // blocks.extend([Tetris::t_default(); 0].into_iter());

    println!("{}", sigils::solve(&mut board, &mut blocks));

    println!("{board}");
}
