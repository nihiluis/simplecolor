#[derive(Clone, Copy, Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    pub fn to_fg_str(&self) -> &str {
        match *self {
            Color::Black => "30",
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
        }
    }

    pub fn to_bg_str(&self) -> &str {
        match *self {
            Color::Black => "40",
            Color::Red => "41",
            Color::Green => "42",
            Color::Yellow => "43",
            Color::Blue => "44",
            Color::Magenta => "45",
            Color::Cyan => "46",
            Color::White => "47",
        }
    }
}

pub trait Colorize<'a> {
    fn color(self, Color) -> String;
    fn bg_color(self, Color) -> String;
}

impl<'a> Colorize<'a> for &'a str {
    fn color(self, color: Color) -> String {
        let fg_str = color.to_fg_str();
        let res = format!("\x1B[{}m{}\x1B[39m", fg_str, self);
        
        res
    }

    fn bg_color(self, color: Color) -> String {
        let bg_str = color.to_bg_str();
        let res = format!("\x1B[{}m{}\x1B[49m", bg_str, self);
        
        res
    }
}