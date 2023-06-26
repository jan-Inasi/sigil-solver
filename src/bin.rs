mod arg_parsing;
// pub mod sigils;

use sigils::board::Board;
use sigils::TetrisSet;

fn main() {
    let cfg = arg_parsing::get_parsed_arguments_or_exit();

    let mut board = Board::new(cfg.width, cfg.height);

    let mut block_set = TetrisSet::from_count_array([
        cfg.o_count,
        cfg.i_count,
        cfg.s_count,
        cfg.z_count,
        cfg.t_count,
        cfg.l_count,
        cfg.j_count,
    ]);

    // let mut board = Board::new(6, 6);
    // let mut blocks = Vec::new();
    // blocks.extend([Tetris::o_default(); 1].into_iter());
    // blocks.extend([Tetris::i_default(); 0].into_iter());
    // blocks.extend([Tetris::s_default(); 4].into_iter());
    // blocks.extend([Tetris::z_default(); 0].into_iter());
    // blocks.extend([Tetris::l_default(); 4].into_iter());
    // blocks.extend([Tetris::j_default(); 0].into_iter());
    // blocks.extend([Tetris::t_default(); 0].into_iter());

    println!("{}", sigils::solve(&mut board, &mut block_set));

    println!("{board}");
}
