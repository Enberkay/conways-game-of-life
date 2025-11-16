use crate::grid::Position;
use macroquad::rand::gen_range;

/// Pattern trait for all Conway's Game of Life patterns
pub trait Pattern {
    /// Returns the name of the pattern
    fn name(&self) -> &'static str;
    
    /// Applies the pattern to the game state
    fn apply(&self, cells: &mut HashSet<Position>, grid_width: i32, grid_height: i32, wrap_world: bool, x: i32, y: i32);
    
    /// Apply pattern with default cell addition logic
    fn add_cell_to_pattern(&self, cells: &mut HashSet<Position>, grid_width: i32, grid_height: i32, wrap_world: bool, x: i32, y: i32) {
        let p = if wrap_world { 
            // Wrap coordinates
            let mut nx = x % grid_width;
            let mut ny = y % grid_height;
            if nx < 0 { nx += grid_width; }
            if ny < 0 { ny += grid_height; }
            Position(nx, ny)
        } else { 
            Position(x, y) 
        };
        
        if wrap_world || ((0..grid_width).contains(&x) && (0..grid_height).contains(&y)) {
            cells.insert(p);
        }
    }
}

/// Pattern for a glider that moves diagonally
pub struct GliderPattern;

impl Pattern for GliderPattern {
    fn name(&self) -> &'static str {
        "Glider"
    }
    
    fn apply(&self, cells: &mut HashSet<Position>, grid_width: i32, grid_height: i32, wrap_world: bool, x: i32, y: i32) {
        for (dx, dy) in [(1,0),(2,1),(0,2),(1,2),(2,2)] { 
            self.add_cell_to_pattern(cells, grid_width, grid_height, wrap_world, x+dx, y+dy); 
        }
    }
}

/// Pattern for randomly placing cells
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
    
    fn apply(&self, cells: &mut HashSet<Position>, grid_width: i32, grid_height: i32, wrap_world: bool, _x: i32, _y: i32) {
        for y in 0..grid_height {
            for x in 0..grid_width {
                if gen_range(0.0, 1.0) < self.density {
                    self.add_cell_to_pattern(cells, grid_width, grid_height, wrap_world, x, y);
                }
            }
        }
    }
}

/// Pattern for a 2x2 block (still life)
pub struct BlockPattern;

impl Pattern for BlockPattern {
    fn name(&self) -> &'static str {
        "Block"
    }
    
    fn apply(&self, cells: &mut HashSet<Position>, grid_width: i32, grid_height: i32, wrap_world: bool, x: i32, y: i32) {
        for dx in 0..2 { 
            for dy in 0..2 { 
                self.add_cell_to_pattern(cells, grid_width, grid_height, wrap_world, x+dx, y+dy); 
            } 
        }
    }
}

/// Pattern for a blinker (oscillator with period 2)
pub struct BlinkerPattern;

impl Pattern for BlinkerPattern {
    fn name(&self) -> &'static str {
        "Blinker"
    }
    
    fn apply(&self, cells: &mut HashSet<Position>, grid_width: i32, grid_height: i32, wrap_world: bool, x: i32, y: i32) {
        for dx in 0..3 { 
            self.add_cell_to_pattern(cells, grid_width, grid_height, wrap_world, x+dx, y); 
        }
    }
}

/// Pattern for a beacon (oscillator with period 2)
pub struct BeaconPattern;

impl Pattern for BeaconPattern {
    fn name(&self) -> &'static str {
        "Beacon"
    }
    
    fn apply(&self, cells: &mut HashSet<Position>, grid_width: i32, grid_height: i32, wrap_world: bool, x: i32, y: i32) {
        for (dx,dy) in [(0,0),(1,0),(0,1),(2,3),(3,2),(3,3)] { 
            self.add_cell_to_pattern(cells, grid_width, grid_height, wrap_world, x+dx, y+dy); 
        }
    }
}

/// Pattern for R-pentomino (chaotic pattern that lasts a long time)
pub struct RPentominoPattern;

impl Pattern for RPentominoPattern {
    fn name(&self) -> &'static str {
        "R-pentomino"
    }
    
    fn apply(&self, cells: &mut HashSet<Position>, grid_width: i32, grid_height: i32, wrap_world: bool, x: i32, y: i32) {
        for (dx,dy) in [(1,0),(2,0),(0,1),(1,1),(1,2)] { 
            self.add_cell_to_pattern(cells, grid_width, grid_height, wrap_world, x+dx, y+dy); 
        }
    }
}

/// Pattern for Acorn (long-lived pattern)
pub struct AcornPattern;

impl Pattern for AcornPattern {
    fn name(&self) -> &'static str {
        "Acorn"
    }
    
    fn apply(&self, cells: &mut HashSet<Position>, grid_width: i32, grid_height: i32, wrap_world: bool, x: i32, y: i32) {
        for (dx,dy) in [(1,0),(3,1),(0,2),(1,2),(4,2),(5,2),(6,2)] { 
            self.add_cell_to_pattern(cells, grid_width, grid_height, wrap_world, x+dx, y+dy); 
        }
    }
}

/// Pattern for Diehard (dies after 130 generations)
pub struct DiehardPattern;

impl Pattern for DiehardPattern {
    fn name(&self) -> &'static str {
        "Diehard"
    }
    
    fn apply(&self, cells: &mut HashSet<Position>, grid_width: i32, grid_height: i32, wrap_world: bool, x: i32, y: i32) {
        for (dx,dy) in [(6,0),(0,1),(1,1),(1,2),(5,2),(6,2),(7,2)] { 
            self.add_cell_to_pattern(cells, grid_width, grid_height, wrap_world, x+dx, y+dy); 
        }
    }
}

/// Pattern for Gosper Glider Gun (periodically emits gliders)
pub struct GosperGunPattern;

impl Pattern for GosperGunPattern {
    fn name(&self) -> &'static str {
        "Gosper Gun"
    }
    
    fn apply(&self, cells: &mut HashSet<Position>, grid_width: i32, grid_height: i32, wrap_world: bool, x: i32, y: i32) {
        let pts = [
            (24,0),(22,1),(24,1),(12,2),(13,2),(20,2),(21,2),(34,2),(35,2),
            (11,3),(15,3),(20,3),(21,3),(34,3),(35,3),(0,4),(1,4),(10,4),(16,4),
            (20,4),(21,4),(0,5),(1,5),(10,5),(14,5),(16,5),(17,5),(22,5),(24,5),
            (10,6),(16,6),(24,6),(11,7),(15,7),(12,8),(13,8),
        ];
        for (dx,dy) in pts { 
            self.add_cell_to_pattern(cells, grid_width, grid_height, wrap_world, x+dx, y+dy); 
        }
    }
}

/// Pattern for Pentadecathlon (oscillator with period 15)
pub struct PentadecathlonPattern;

impl Pattern for PentadecathlonPattern {
    fn name(&self) -> &'static str {
        "Pentadecathlon"
    }
    
    fn apply(&self, cells: &mut HashSet<Position>, grid_width: i32, grid_height: i32, wrap_world: bool, x: i32, y: i32) {
        for (dx,dy) in [(0,0),(1,0),(2,0),(3,0),(1,-1),(1,1),(4,-1),(4,1),(5,0),(6,0),(7,0),(8,0)] {
            self.add_cell_to_pattern(cells, grid_width, grid_height, wrap_world, x+dx, y+dy);
        }
    }
}

/// Get a pattern by index for menu selection
pub fn get_pattern_by_index(index: usize) -> Box<dyn Pattern> {
    match index {
        0 => Box::new(GliderPattern),
        1 => Box::new(RandomPattern::new(crate::config::RANDOM_DENSITY)),
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

/// Get all pattern names for the menu
pub fn get_pattern_names() -> Vec<&'static str> {
    vec![
        "Glider", "Random", "Block", "Blinker", "Beacon",
        "R-pentomino", "Acorn", "Diehard", "Gosper Gun", "Pentadecathlon",
    ]
}