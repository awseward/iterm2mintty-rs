extern crate clap;
extern crate plist;
#[macro_use]
extern crate serde_derive;
extern crate serde_ini;

use plist::serde::deserialize;
use std::fs::File;
use clap::{App, Arg};

mod iterm2;
mod mintty;

fn main() {
    let matches = App::new("iterm2mintty")
        .version("1.0")
        .about("Converts iterm2 themes to mintty format")
        .author("bobcats")
        .arg(
            Arg::with_name("iterm2theme").index(1).required(true)
        )
        .get_matches();

    let file_path = matches.value_of("iterm2theme").unwrap();
    let file = File::open(file_path).unwrap();
    let theme: iterm2::Theme = deserialize(file).unwrap();
    let mintty_theme: mintty::Theme = mintty::Theme::from(theme);
    let ini = serde_ini::to_string(&mintty_theme).unwrap();
    println!("{}", &ini);
}
