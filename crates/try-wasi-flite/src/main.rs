use std::env;

use flite::lexicon::lexicon;

fn main() {
    let arg = env::args().nth(1).unwrap();
    let lexes = lexicon().lookup(&arg, None).unwrap();
    println!("{lexes:?}")
}
