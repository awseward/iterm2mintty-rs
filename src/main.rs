extern crate plist;
#[macro_use]
extern crate serde_derive;
extern crate serde_ini;

use plist::serde::deserialize;
use std::fs::File;

mod iterm2;
mod mintty;

fn main() {
    let file = File::open("./tests/data/Hybrid.itermcolors").unwrap();
    let theme: iterm2::Theme = deserialize(file).unwrap();
    println!("{:?}", &theme);
    let mintty_theme: mintty::Theme = mintty::Theme::from(theme);
    println!("{:?}", &mintty_theme);
    let ini = serde_ini::to_string(&mintty_theme).unwrap();
    println!("{}", &ini);
}
