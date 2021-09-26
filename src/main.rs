use clap::Clap;
use emoji_pix::{emojify, Opts};

fn main() {
    let opts = Opts::parse();
    let converted = emojify(opts);
    print!("{}", &converted);
}
