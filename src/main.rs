extern crate clap;
extern crate plist;
#[macro_use]
extern crate serde_derive;
extern crate serde_ini;
extern crate serde_yaml;

use clap::{App, Arg};
use plist::serde::deserialize;
use std::fs::File;

mod alacritty;
mod iterm2;
mod mintty;

const FILE_NOT_FOUND_ERROR_MSG: &str = "iterm2theme could not be opened.";

const DESERIALIZE_ERROR_MSG: &str = r#"""
    iterm2theme could not be parsed.
    
    If you think you have a valid iterm2theme, please post an 
    issue on the https://github.com/bobcats/iterm2mintty-rs/issues");
"""#;

fn main() {
    let matches = App::new("iterm2mintty")
        .version("0.2.0")
        .about("Converts iterm2 themes to mintty format")
        .author("bobcats")
        .arg(Arg::with_name("iterm2theme").index(1).required(true))
        .arg(Arg::with_name("alacritty").short("a").help("Sets destination theme format to alacritty format"))
        .get_matches();

    let file_path = matches.value_of("iterm2theme").unwrap();
    let file = File::open(file_path).expect(FILE_NOT_FOUND_ERROR_MSG);
    let alacritty = matches.is_present("alacritty");
    let theme: iterm2::Theme = deserialize(file).expect(DESERIALIZE_ERROR_MSG);
    let output = if alacritty {
        let alacritty_theme: alacritty::Theme = alacritty::Theme::from(theme);

        serde_yaml::to_string(&alacritty_theme).unwrap()
    } else {
        let mintty_theme: mintty::Theme = mintty::Theme::from(theme);

        serde_ini::to_string(&mintty_theme).unwrap()
    };

    println!("{}", &output);
}
