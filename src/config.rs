// Game configuration constants
pub const CELL_SIZE: i32 = 10;          // Pixel size of each cell
pub const RANDOM_DENSITY: f32 = 0.20;     // Density for random patterns
pub const SPEED_MIN: f32 = 1.0;           // Minimum generations per second
pub const SPEED_MAX: f32 = 120.0;         // Maximum generations per second
pub const SPEED_INIT: f32 = 10.0;         // Default generations per second

// Available screen resolutions (width, height)
pub const SCREEN_SIZES: [(i32, i32); 5] = [
    (640, 480),
    (800, 600),
    (1024, 768),
    (1280, 720),
    (1920, 1080),
];