use std::collections::HashSet;
use macroquad::prelude::*;

use crate::grid::{Grid, Position};
use crate::patterns::{Pattern, PatternContext};
use crate::themes::ColorTheme;

/// Core game state for Conway's Game of Life simulation
pub struct GameOfLife {
    pub live: HashSet<Position>,
    pub grid: Grid,
    pub cell: i32,          // Visual size of each cell in pixels
    pub generation: u64,     // Current generation count
    pub show_grid: bool,     // Whether to draw grid lines
    pub theme: ColorTheme,   // Current color theme
}

impl GameOfLife {
    /// Create a new game grid with specified dimensions
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

    /// Add a live cell at the specified position
    pub fn add_cell(&mut self, x: i32, y: i32) {
        let p = if self.grid.wrap_world { self.grid.wrap(x, y) } else { Position(x, y) };
        if self.grid.wrap_world || self.grid.in_bounds(p.x(), p.y()) {
            self.live.insert(p);
        }
    }

    /// Toggle a cell between alive and dead states
    pub fn toggle_cell(&mut self, x: i32, y: i32) {
        let p = if self.grid.wrap_world { self.grid.wrap(x, y) } else { Position(x, y) };
        if !(self.grid.wrap_world || self.grid.in_bounds(p.x(), p.y())) { return; }
        if !self.live.remove(&p) { self.live.insert(p); }
    }

    /// Remove all cells and reset generation count to zero
    pub fn clear(&mut self) {
        self.live.clear();
        self.generation = 0;
    }

    /// Randomly distribute cells across the grid
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

    /// Apply a pattern at the specified position
    pub fn apply_pattern(&mut self, pattern: &dyn Pattern, x: i32, y: i32) {
        let mut ctx = PatternContext {
            cells: &mut self.live,
            grid_width: self.grid.width,
            grid_height: self.grid.height,
            wrap_world: self.grid.wrap_world,
        };
        
        pattern.apply(&mut ctx, x, y);
    }

    /// Switch to the next available color theme
    pub fn cycle_theme(&mut self) {
        self.theme = match self.theme {
            ColorTheme::Classic => ColorTheme::Dark,
            ColorTheme::Dark => ColorTheme::Pastel,
            ColorTheme::Pastel => ColorTheme::Neon,
            ColorTheme::Neon => ColorTheme::Classic,
        };
    }

    /// Draw the current game state to screen
    pub fn draw(&self) {
        let colors = self.theme.colors();
        clear_background(colors.background);

        // Draw all living cells
        for &Position(x, y) in &self.live {
            draw_rectangle(
                (x * self.cell) as f32,
                (y * self.cell) as f32,
                self.cell as f32,
                self.cell as f32,
                colors.cell,
            );
        }

        // Draw grid lines if enabled
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

        // Draw game border
        draw_rectangle_lines(
            0.0, 0.0,
            (self.grid.width * self.cell) as f32,
            (self.grid.height * self.cell) as f32,
            3.0, colors.border,
        );
    }

    /// Draw heads-up display with game information
    pub fn draw_hud(&self, paused: bool, speed: f32) {
        let colors = self.theme.colors();
        // Display game statistics and controls
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

        let help = "Controls: Space:Pause | N:Step | -/=:Speed | R:Random | C:Clear | G:Grid | W:Wrap | T:Theme | Esc:Menu | Mouse:Draw/Erase";
        draw_text(help, 10.0, 46.0, 18.0, colors.text_secondary);
    }
}