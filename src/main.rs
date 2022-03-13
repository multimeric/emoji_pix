use clap::Parser;
use emoji_pix::{emojify, Opts};

fn main() {
    let opts = Opts::parse();
    let converted = emojify(opts);
    print!("{}", &converted);
}
