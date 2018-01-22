extern crate plist;
#[macro_use]
extern crate serde_derive;
extern crate serde_ini;

use plist::Plist;
use plist::serde::deserialize;
use std::fs::File;
use std::fmt;

#[derive(Debug, Deserialize)]
struct Iterm2Color {
    #[serde(rename = "Blue Component")]
    blue: f64,
    #[serde(rename = "Red Component")]
    red: f64,
    #[serde(rename = "Green Component")]
    green: f64,
}

#[derive(Debug, Deserialize)]
struct Iterm2Theme {
    #[serde(rename = "Ansi 0 Color")]
    black: Iterm2Color,
    #[serde(rename = "Ansi 1 Color")]
    red: Iterm2Color,
    #[serde(rename = "Ansi 2 Color")]
    green: Iterm2Color,
    #[serde(rename = "Ansi 3 Color")]
    yellow: Iterm2Color,
    #[serde(rename = "Ansi 4 Color")]
    blue: Iterm2Color,
    #[serde(rename = "Ansi 5 Color")]
    magenta: Iterm2Color,
    #[serde(rename = "Ansi 6 Color")]
    cyan: Iterm2Color,
    #[serde(rename = "Ansi 7 Color")]
    white: Iterm2Color,
    #[serde(rename = "Ansi 8 Color")]
    bright_black: Iterm2Color,
    #[serde(rename = "Ansi 9 Color")]
    bright_red: Iterm2Color,
    #[serde(rename = "Ansi 10 Color")]
    bright_green: Iterm2Color,
    #[serde(rename = "Ansi 11 Color")]
    bright_yellow: Iterm2Color,
    #[serde(rename = "Ansi 12 Color")]
    bright_blue: Iterm2Color,
    #[serde(rename = "Ansi 13 Color")]
    bright_magenta: Iterm2Color,
    #[serde(rename = "Ansi 14 Color")]
    bright_cyan: Iterm2Color,
    #[serde(rename = "Ansi 15 Color")]
    bright_white: Iterm2Color,
    #[serde(rename = "Background Color")]
    background: Iterm2Color,
}

#[derive(Debug, Serialize)]
struct MinttyColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl From<Iterm2Color> for MinttyColor {
    fn from(iterm2color: Iterm2Color) -> Self {
        MinttyColor {
            red: (iterm2color.red * 255.0) as u8,
            green: (iterm2color.green * 255.0) as u8,
            blue: (iterm2color.blue * 255.0) as u8,
        }
    }
}

impl fmt::Display for MinttyColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.red, self.green, self.blue)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct MinttyTheme {
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String,
    bold_black: String,
    bold_red: String,
    bold_green: String,
    bold_yellow: String,
    bold_blue: String,
    bold_magenta: String,
    bold_cyan: String,
    bold_white: String,
    background_colour: String,
}

impl From<Iterm2Theme> for MinttyTheme {
    fn from(iterm2: Iterm2Theme) -> Self {
        MinttyTheme {
            black: format!("{}", MinttyColor::from(iterm2.black)),
            red: format!("{}", MinttyColor::from(iterm2.red)),
            green: format!("{}", MinttyColor::from(iterm2.green)),
            yellow: format!("{}", MinttyColor::from(iterm2.yellow)),
            blue: format!("{}", MinttyColor::from(iterm2.blue)),
            magenta: format!("{}", MinttyColor::from(iterm2.magenta)),
            cyan: format!("{}", MinttyColor::from(iterm2.cyan)),
            white: format!("{}", MinttyColor::from(iterm2.white)),
            bold_black: format!("{}", MinttyColor::from(iterm2.bright_black)),
            bold_red: format!("{}", MinttyColor::from(iterm2.bright_red)),
            bold_green: format!("{}", MinttyColor::from(iterm2.bright_green)),
            bold_yellow: format!("{}", MinttyColor::from(iterm2.bright_yellow)),
            bold_blue: format!("{}", MinttyColor::from(iterm2.bright_blue)),
            bold_magenta: format!("{}", MinttyColor::from(iterm2.bright_magenta)),
            bold_cyan: format!("{}", MinttyColor::from(iterm2.bright_cyan)),
            bold_white: format!("{}", MinttyColor::from(iterm2.bright_white)),
            background_colour: format!("{}", MinttyColor::from(iterm2.background)),
        }
    }
}

fn main() {
    let file = File::open("./tests/data/Hybrid.itermcolors").unwrap();
    // let plist = Plist::read(file).unwrap();
    // println!("{:?}", plist);
    let theme: Iterm2Theme = deserialize(file).unwrap();
    println!("{:?}", &theme);
    let mintty_theme : MinttyTheme = MinttyTheme::from(theme);
    println!("{:?}", &mintty_theme);
    let ini = serde_ini::to_string(&mintty_theme).unwrap();
    println!("{:?}", &ini);
}
