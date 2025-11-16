use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorTheme {
    #[default]
    Classic,  // Original green
    Dark,     // Dark theme (white on black)
    Pastel,   // Pastel theme
    Neon,     // Neon theme
}

pub struct ThemeColors {
    pub background: Color,
    pub cell: Color,
    pub grid: Color,
    pub border: Color,
    pub text: Color,
    pub text_secondary: Color,
}

impl ColorTheme {
    pub fn colors(&self) -> ThemeColors {
        match self {
            ColorTheme::Classic => ThemeColors {
                background: BLACK,
                cell: GREEN,
                grid: Color::new(0.15, 0.15, 0.15, 1.0),
                border: RED,
                text: WHITE,
                text_secondary: GRAY,
            },
            ColorTheme::Dark => ThemeColors {
                background: BLACK,
                cell: WHITE,
                grid: Color::new(0.2, 0.2, 0.2, 1.0),
                border: Color::new(0.8, 0.8, 0.8, 1.0),
                text: WHITE,
                text_secondary: Color::new(0.7, 0.7, 0.7, 1.0),
            },
            ColorTheme::Pastel => ThemeColors {
                background: Color::new(0.95, 0.95, 0.98, 1.0),
                cell: Color::new(0.8, 0.6, 0.9, 1.0),  // Light purple
                grid: Color::new(0.85, 0.85, 0.85, 1.0),
                border: Color::new(0.6, 0.4, 0.8, 1.0),
                text: Color::new(0.2, 0.2, 0.3, 1.0),
                text_secondary: Color::new(0.4, 0.4, 0.5, 1.0),
            },
            ColorTheme::Neon => ThemeColors {
                background: Color::new(0.05, 0.05, 0.1, 1.0),  // Dark blue
                cell: Color::new(0.0, 1.0, 0.8, 1.0),  // Neon green
                grid: Color::new(0.2, 0.2, 0.4, 1.0),
                border: Color::new(1.0, 0.0, 0.8, 1.0),  // Pink
                text: Color::new(0.8, 1.0, 1.0, 1.0),
                text_secondary: Color::new(0.6, 0.8, 1.0, 1.0),
            },
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            ColorTheme::Classic => "Classic",
            ColorTheme::Dark => "Dark",
            ColorTheme::Pastel => "Pastel",
            ColorTheme::Neon => "Neon",
        }
    }
}