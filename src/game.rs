use std::collections::HashSet;
use macroquad::prelude::*;

use crate::grid::{Grid, Position};
use crate::patterns::{Pattern, PatternContext};
use crate::themes::ColorTheme;

/// Main game state for Conway's Game of Life
pub struct GameOfLife {
    pub live: HashSet<Position>,
    pub grid: Grid,
    pub cell: i32,
    pub generation: u64,
    pub show_grid: bool,
    pub theme: ColorTheme,
}

impl GameOfLife {
    /// Create a new game with the specified dimensions
    pub fn new(width: i32, height: i32, cell_size: i32) -> Self {
        Self {
            live: HashSet::new(),
            grid: Grid::new(width, height),
            cell: cell_size,
            generation: 0,
            show_grid: true,
            theme: ColorTheme::Classic,
        }
    }

    /// Add a cell at the specified position
    pub fn add_cell(&mut self, x: i32, y: i32) {
        let p = if self.grid.wrap_world { self.grid.wrap(x, y) } else { Position(x, y) };
        if self.grid.wrap_world || self.grid.in_bounds(p.x(), p.y()) {
            self.live.insert(p);
        }
    }

    /// Toggle a cell at the specified position
    pub fn toggle_cell(&mut self, x: i32, y: i32) {
        let p = if self.grid.wrap_world { self.grid.wrap(x, y) } else { Position(x, y) };
        if !(self.grid.wrap_world || self.grid.in_bounds(p.x(), p.y())) { return; }
        if !self.live.remove(&p) { self.live.insert(p); }
    }

    /// Clear all cells and reset generation counter
    pub fn clear(&mut self) {
        self.live.clear();
        self.generation = 0;
    }

    /// Fill the grid randomly with cells
    pub fn random_fill(&mut self, density: f32) {
        use macroquad::rand::gen_range;
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                if gen_range(0.0, 1.0) < density {
                    self.add_cell(x, y);
                }
            }
        }
    }

    /// Calculate the next generation of cells
    pub fn next_generation(&mut self) {
        self.live = self.grid.next_generation(&self.live);
        self.generation += 1;
    }

    /// Apply a pattern to the game
    pub fn apply_pattern(&mut self, pattern: &dyn Pattern, x: i32, y: i32) {
        let mut ctx = PatternContext {
            cells: &mut self.live,
            grid_width: self.grid.width,
            grid_height: self.grid.height,
            wrap_world: self.grid.wrap_world,
        };
        
        pattern.apply(&mut ctx, x, y);
    }

    /// Cycle to the next color theme
    pub fn cycle_theme(&mut self) {
        self.theme = match self.theme {
            ColorTheme::Classic => ColorTheme::Dark,
            ColorTheme::Dark => ColorTheme::Pastel,
            ColorTheme::Pastel => ColorTheme::Neon,
            ColorTheme::Neon => ColorTheme::Classic,
        };
    }

    /// Render the game
    pub fn draw(&self) {
        let colors = self.theme.colors();
        clear_background(colors.background);

        // Draw live cells
        for &Position(x, y) in &self.live {
            draw_rectangle(
                (x * self.cell) as f32,
                (y * self.cell) as f32,
                self.cell as f32,
                self.cell as f32,
                colors.cell,
            );
        }

        // Draw grid lines
        if self.show_grid {
            for x in 0..=self.grid.width {
                draw_line(
                    (x * self.cell) as f32, 0.0,
                    (x * self.cell) as f32, (self.grid.height * self.cell) as f32,
                    1.0, colors.grid,
                );
            }
            for y in 0..=self.grid.height {
                draw_line(
                    0.0, (y * self.cell) as f32,
                    (self.grid.width * self.cell) as f32, (y * self.cell) as f32,
                    1.0, colors.grid,
                );
            }
        }

        // Draw border
        draw_rectangle_lines(
            0.0, 0.0,
            (self.grid.width * self.cell) as f32,
            (self.grid.height * self.cell) as f32,
            3.0, colors.border,
        );
    }

    /// Draw the HUD overlay
    pub fn draw_hud(&self, paused: bool, speed: f32) {
        let colors = self.theme.colors();
        let info = format!(
            "Gen:{} | FPS:{:.0} | {} | speed:{:.1} gen/s | grid:{} | wrap:{} | Theme:{}",
            self.generation, get_fps() as f32,
            if paused { "PAUSED" } else { "RUN" },
            speed,
            if self.show_grid { "on" } else { "off" },
            if self.grid.wrap_world { "on" } else { "off" },
            self.theme.name(),
        );
        draw_text(&info, 10.0, 22.0, 22.0, colors.text);

        let help = "Space: Pause | N: Step | -/=: Speed | R: Random | C: Clear | G: Grid | W: Wrap | T: Theme | Esc: Menu | LMB: Draw/Erase";
        draw_text(help, 10.0, 46.0, 18.0, colors.text_secondary);
    }
}