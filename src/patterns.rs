use std::collections::HashSet;
use crate::grid::Position;
use crate::config::RANDOM_DENSITY;
use macroquad::rand::gen_range;

/// Context for pattern application with grid information
pub struct PatternContext<'a> {
    pub cells: &'a mut HashSet<Position>,
    pub grid_width: i32,
    pub grid_height: i32,
    pub wrap_world: bool,
}

impl<'a> PatternContext<'a> {
    /// Add a cell with edge wrapping if enabled
    pub fn add_cell(&mut self, x: i32, y: i32) {
        let p = if self.wrap_world { 
            // Wrap coordinates around edges
            let mut nx = x % self.grid_width;
            let mut ny = y % self.grid_height;
            if nx < 0 { nx += self.grid_width; }
            if ny < 0 { ny += self.grid_height; }
            Position::new(nx, ny)
        } else { 
            Position::new(x, y) 
        };
        
        if self.wrap_world || ((0..self.grid_width).contains(&x) && (0..self.grid_height).contains(&y)) {
            self.cells.insert(p);
        }
    }
}

/// Pattern trait for all Conway's Game of Life patterns
pub trait Pattern {
    /// Returns the name of the pattern
    fn name(&self) -> &'static str;
    
    /// Applies the pattern to the game state
    fn apply(&self, ctx: &mut PatternContext, x: i32, y: i32);
}

/// A glider that moves diagonally across the grid
pub struct GliderPattern;

impl Pattern for GliderPattern {
    fn name(&self) -> &'static str {
        "Glider"
    }
    
    fn apply(&self, ctx: &mut PatternContext, x: i32, y: i32) {
        for (dx, dy) in [(1,0),(2,1),(0,2),(1,2),(2,2)] { 
            ctx.add_cell(x+dx, y+dy); 
        }
    }
}

/// Randomly distributes cells across the grid
pub struct RandomPattern {
    pub density: f32,
}

impl RandomPattern {
    pub fn new(density: f32) -> Self {
        Self { density }
    }
}

impl Pattern for RandomPattern {
    fn name(&self) -> &'static str {
        "Random"
    }
    
    fn apply(&self, ctx: &mut PatternContext, _x: i32, _y: i32) {
        for y in 0..ctx.grid_height {
            for x in 0..ctx.grid_width {
                if gen_range(0.0, 1.0) < self.density {
                    ctx.add_cell(x, y);
                }
            }
        }
    }
}

/// A stable 2x2 block pattern (still life)
pub struct BlockPattern;

impl Pattern for BlockPattern {
    fn name(&self) -> &'static str {
        "Block"
    }
    
    fn apply(&self, ctx: &mut PatternContext, x: i32, y: i32) {
        for dx in 0..2 { 
            for dy in 0..2 { 
                ctx.add_cell(x+dx, y+dy); 
            } 
        }
    }
}

/// A 3-cell line that oscillates with period 2
pub struct BlinkerPattern;

impl Pattern for BlinkerPattern {
    fn name(&self) -> &'static str {
        "Blinker"
    }
    
    fn apply(&self, ctx: &mut PatternContext, x: i32, y: i32) {
        for dx in 0..3 { 
            ctx.add_cell(x+dx, y); 
        }
    }
}

/// Two 2x2 blocks that alternate flashing (oscillator)
pub struct BeaconPattern;

impl Pattern for BeaconPattern {
    fn name(&self) -> &'static str {
        "Beacon"
    }
    
    fn apply(&self, ctx: &mut PatternContext, x: i32, y: i32) {
        for (dx,dy) in [(0,0),(1,0),(0,1),(2,3),(3,2),(3,3)] { 
            ctx.add_cell(x+dx, y+dy); 
        }
    }
}

/// A 5-cell pattern that evolves chaotically for many generations
pub struct RPentominoPattern;

impl Pattern for RPentominoPattern {
    fn name(&self) -> &'static str {
        "R-pentomino"
    }
    
    fn apply(&self, ctx: &mut PatternContext, x: i32, y: i32) {
        for (dx,dy) in [(1,0),(2,0),(0,1),(1,1),(1,2)] { 
            ctx.add_cell(x+dx, y+dy); 
        }
    }
}

/// A 7-cell sparse pattern that grows into a large field
pub struct AcornPattern;

impl Pattern for AcornPattern {
    fn name(&self) -> &'static str {
        "Acorn"
    }
    
    fn apply(&self, ctx: &mut PatternContext, x: i32, y: i32) {
        for (dx,dy) in [(1,0),(3,1),(0,2),(1,2),(4,2),(5,2),(6,2)] { 
            ctx.add_cell(x+dx, y+dy); 
        }
    }
}

/// A 7-cell pattern that disappears after exactly 130 generations
pub struct DiehardPattern;

impl Pattern for DiehardPattern {
    fn name(&self) -> &'static str {
        "Diehard"
    }
    
    fn apply(&self, ctx: &mut PatternContext, x: i32, y: i32) {
        for (dx,dy) in [(6,0),(0,1),(1,1),(1,2),(5,2),(6,2),(7,2)] { 
            ctx.add_cell(x+dx, y+dy); 
        }
    }
}

/// First discovered pattern that creates gliders indefinitely
pub struct GosperGunPattern;

impl Pattern for GosperGunPattern {
    fn name(&self) -> &'static str {
        "Gosper Gun"
    }
    
    fn apply(&self, ctx: &mut PatternContext, x: i32, y: i32) {
        let pts = [
            (24,0),(22,1),(24,1),(12,2),(13,2),(20,2),(21,2),(34,2),(35,2),
            (11,3),(15,3),(20,3),(21,3),(34,3),(35,3),(0,4),(1,4),(10,4),(16,4),
            (20,4),(21,4),(0,5),(1,5),(10,5),(14,5),(16,5),(17,5),(22,5),(24,5),
            (10,6),(16,6),(24,6),(11,7),(15,7),(12,8),(13,8),
        ];
        for (dx,dy) in pts { 
            ctx.add_cell(x+dx, y+dy); 
        }
    }
}

/// A long oscillator with period 15 generations
pub struct PentadecathlonPattern;

impl Pattern for PentadecathlonPattern {
    fn name(&self) -> &'static str {
        "Pentadecathlon"
    }
    
    fn apply(&self, ctx: &mut PatternContext, x: i32, y: i32) {
        for (dx,dy) in [(0,0),(1,0),(2,0),(3,0),(1,-1),(1,1),(4,-1),(4,1),(5,0),(6,0),(7,0),(8,0)] {
            ctx.add_cell(x+dx, y+dy);
        }
    }
}

/// Get pattern instance by index for menu selection
pub fn get_pattern_by_index(index: usize) -> Box<dyn Pattern> {
    match index {
        0 => Box::new(GliderPattern),
        1 => Box::new(RandomPattern::new(RANDOM_DENSITY)),
        2 => Box::new(BlockPattern),
        3 => Box::new(BlinkerPattern),
        4 => Box::new(BeaconPattern),
        5 => Box::new(RPentominoPattern),
        6 => Box::new(AcornPattern),
        7 => Box::new(DiehardPattern),
        8 => Box::new(GosperGunPattern),
        9 => Box::new(PentadecathlonPattern),
        _ => Box::new(GliderPattern), // Default to glider
    }
}

