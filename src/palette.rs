use bevy::prelude::Color;

pub enum ColorPalette {
    Background,
    SnakeColor1,
    SnakeColor2,
    GroundColor,
}

impl ColorPalette {
    pub fn color(&self) -> Color {
        match *self {
            Self::Background => Color::hex("3ba481").unwrap(),
            Self::SnakeColor1 => Color::hex("7ac88b").unwrap(),
            Self::SnakeColor2 => Color::hex("cafe81").unwrap(),
            Self::GroundColor => Color::hex("fbefa3").unwrap(),
        }
    }
}
