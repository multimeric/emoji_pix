use clap::Clap;
use pixel_art_emoji::{emojify, Opts};

fn main() {
    let opts = Opts::parse();
    let converted = emojify(opts);
    print!("{}", &converted);
}
