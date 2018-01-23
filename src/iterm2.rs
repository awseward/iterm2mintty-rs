#[derive(Debug, Deserialize)]
pub struct Color {
    #[serde(rename = "Blue Component")]
    pub blue: f64,
    #[serde(rename = "Red Component")]
    pub red: f64,
    #[serde(rename = "Green Component")]
    pub green: f64,
}

#[derive(Debug, Deserialize)]
pub struct Theme {
    #[serde(rename = "Ansi 0 Color")]
    pub black: Color,
    #[serde(rename = "Ansi 1 Color")]
    pub red: Color,
    #[serde(rename = "Ansi 2 Color")]
    pub green: Color,
    #[serde(rename = "Ansi 3 Color")]
    pub yellow: Color,
    #[serde(rename = "Ansi 4 Color")]
    pub blue: Color,
    #[serde(rename = "Ansi 5 Color")]
    pub magenta: Color,
    #[serde(rename = "Ansi 6 Color")]
    pub cyan: Color,
    #[serde(rename = "Ansi 7 Color")]
    pub white: Color,
    #[serde(rename = "Ansi 8 Color")]
    pub bright_black: Color,
    #[serde(rename = "Ansi 9 Color")]
    pub bright_red: Color,
    #[serde(rename = "Ansi 10 Color")]
    pub bright_green: Color,
    #[serde(rename = "Ansi 11 Color")]
    pub bright_yellow: Color,
    #[serde(rename = "Ansi 12 Color")]
    pub bright_blue: Color,
    #[serde(rename = "Ansi 13 Color")]
    pub bright_magenta: Color,
    #[serde(rename = "Ansi 14 Color")]
    pub bright_cyan: Color,
    #[serde(rename = "Ansi 15 Color")]
    pub bright_white: Color,
    #[serde(rename = "Background Color")]
    pub background: Color,
}
