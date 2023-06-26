use argparse::{ArgumentParser, Store};

pub struct Config {
    pub width: u8,
    pub height: u8,
    pub o_count: u8,
    pub i_count: u8,
    pub s_count: u8,
    pub z_count: u8,
    pub l_count: u8,
    pub j_count: u8,
    pub t_count: u8,
}

pub fn get_parsed_arguments_or_exit() -> Config {
    let mut cfg = Config {
        width: 0,
        height: 0,
        o_count: 0,
        i_count: 0,
        s_count: 0,
        z_count: 0,
        l_count: 0,
        j_count: 0,
        t_count: 0,
    };
    {
        let mut ap = ArgumentParser::new();
        ap.set_description(
            "are you a human? well I'm certainly a human who doesn't like solving sigil puzzles",
        );
        ap.refer(&mut cfg.width).required().add_argument(
            "width",
            Store,
            "specify sigil board width",
        );
        ap.refer(&mut cfg.height).required().add_argument(
            "height",
            Store,
            "specify sigil board height",
        );
        ap.refer(&mut cfg.o_count).add_option(
            &["-o", "-O", "--square"],
            Store,
            "specify the number of O shaped sigils",
        );
        ap.refer(&mut cfg.i_count).add_option(
            &["-i", "-I", "--line"],
            Store,
            "specify the number of I shaped sigils",
        );
        ap.refer(&mut cfg.s_count).add_option(
            &["-s", "-S", "--s_like"],
            Store,
            "specify the number of S shaped sigils",
        );
        ap.refer(&mut cfg.z_count).add_option(
            &["-z", "-Z", "--z_like"],
            Store,
            "specify the number of Z shaped sigils",
        );
        ap.refer(&mut cfg.l_count).add_option(
            &["-l", "-L", "--l_like"],
            Store,
            "specify the number of L shaped sigils",
        );
        ap.refer(&mut cfg.j_count).add_option(
            &["-j", "-J", "--j_like"],
            Store,
            "specify the number of J shaped sigils",
        );
        ap.refer(&mut cfg.t_count).add_option(
            &["-t", "-T", "--t_like"],
            Store,
            "specify the number of T shaped sigils",
        );
        ap.parse_args_or_exit();
    }

    cfg
}
