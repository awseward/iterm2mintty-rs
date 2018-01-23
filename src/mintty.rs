use iterm2;
use std::fmt;

#[derive(Debug, Serialize)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl From<iterm2::Color> for Color {
    fn from(iterm2color: iterm2::Color) -> Self {
        Color {
            red: (iterm2color.red * 255.0) as u8,
            green: (iterm2color.green * 255.0) as u8,
            blue: (iterm2color.blue * 255.0) as u8,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.red, self.green, self.blue)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Theme {
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

impl From<iterm2::Theme> for Theme {
    fn from(iterm2: iterm2::Theme) -> Self {
        Theme {
            black: format!("{}", Color::from(iterm2.black)),
            red: format!("{}", Color::from(iterm2.red)),
            green: format!("{}", Color::from(iterm2.green)),
            yellow: format!("{}", Color::from(iterm2.yellow)),
            blue: format!("{}", Color::from(iterm2.blue)),
            magenta: format!("{}", Color::from(iterm2.magenta)),
            cyan: format!("{}", Color::from(iterm2.cyan)),
            white: format!("{}", Color::from(iterm2.white)),
            bold_black: format!("{}", Color::from(iterm2.bright_black)),
            bold_red: format!("{}", Color::from(iterm2.bright_red)),
            bold_green: format!("{}", Color::from(iterm2.bright_green)),
            bold_yellow: format!("{}", Color::from(iterm2.bright_yellow)),
            bold_blue: format!("{}", Color::from(iterm2.bright_blue)),
            bold_magenta: format!("{}", Color::from(iterm2.bright_magenta)),
            bold_cyan: format!("{}", Color::from(iterm2.bright_cyan)),
            bold_white: format!("{}", Color::from(iterm2.bright_white)),
            background_colour: format!("{}", Color::from(iterm2.background)),
        }
    }
}
