use std::io::{self, BufReader};

use filler::game;

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut stdout = io::stdout();

    if let Err(err) = game::run_game(&mut reader, &mut stdout) {
        eprintln!("filler: {err}");
        std::process::exit(1);
    }
}
