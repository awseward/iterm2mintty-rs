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
    write!(f, "#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
  }
}

#[derive(Debug, Serialize)]
pub struct Theme {
  primary: Primary,
  normal: Normal,
  bright: Bright,
  cursor: Cursor,
}

#[derive(Debug, Serialize)]
pub struct Primary {
  foreground: String,
  background: String,
}

#[derive(Debug, Serialize)]
pub struct Normal {
  black: String,
  red: String,
  green: String,
  yellow: String,
  blue: String,
  magenta: String,
  cyan: String,
  white: String,
}

#[derive(Debug, Serialize)]
pub struct Bright {
  black: String,
  red: String,
  green: String,
  yellow: String,
  blue: String,
  magenta: String,
  cyan: String,
  white: String,
}

#[derive(Debug, Serialize)]
pub struct Cursor {
  cursor: String,
  text: String,
}

impl From<iterm2::Theme> for Theme {
  fn from(iterm2: iterm2::Theme) -> Self {
    Theme {
      primary: Primary {
        background: format!("{}", Color::from(iterm2.background)),
        foreground: format!("{}", Color::from(iterm2.foreground)),
      },
      normal: Normal {
        black: format!("{}", Color::from(iterm2.black)),
        red: format!("{}", Color::from(iterm2.red)),
        green: format!("{}", Color::from(iterm2.green)),
        yellow: format!("{}", Color::from(iterm2.yellow)),
        blue: format!("{}", Color::from(iterm2.blue)),
        magenta: format!("{}", Color::from(iterm2.magenta)),
        cyan: format!("{}", Color::from(iterm2.cyan)),
        white: format!("{}", Color::from(iterm2.white)),
      },
      bright: Bright {
        black: format!("{}", Color::from(iterm2.bright_black)),
        red: format!("{}", Color::from(iterm2.bright_red)),
        green: format!("{}", Color::from(iterm2.bright_green)),
        yellow: format!("{}", Color::from(iterm2.bright_yellow)),
        blue: format!("{}", Color::from(iterm2.bright_blue)),
        magenta: format!("{}", Color::from(iterm2.bright_magenta)),
        cyan: format!("{}", Color::from(iterm2.bright_cyan)),
        white: format!("{}", Color::from(iterm2.bright_white)),
      },
      cursor: Cursor {
        cursor: format!("{}", Color::from(iterm2.cursor)),
        text: format!("{}", Color::from(iterm2.cursor_text)),
      }
    }
  }
}
