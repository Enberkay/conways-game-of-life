use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::collections::{HashMap, HashSet};

// ========== Config ==========
const CELL_SIZE: i32 = 10;
const RANDOM_DENSITY: f32 = 0.20;
const SPEED_MIN: f32 = 1.0;
const SPEED_MAX: f32 = 120.0;
const SPEED_INIT: f32 = 10.0;

// ========== Color Themes ==========
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum ColorTheme {
    #[default]
    Classic,  // Original green
    Dark,     // Dark theme (white on black)
    Pastel,   // Pastel theme
    Neon,     // Neon theme
}

impl ColorTheme {
    fn colors(&self) -> ThemeColors {
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
    
    fn name(&self) -> &'static str {
        match self {
            ColorTheme::Classic => "Classic",
            ColorTheme::Dark => "Dark",
            ColorTheme::Pastel => "Pastel",
            ColorTheme::Neon => "Neon",
        }
    }
}

struct ThemeColors {
    background: Color,
    cell: Color,
    grid: Color,
    border: Color,
    text: Color,
    text_secondary: Color,
}

// ========== Core Types ==========
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position(i32, i32);

impl Position {
    #[inline]
    fn offset(self, dx: i32, dy: i32) -> Self {
        Position(self.0 + dx, self.1 + dy)
    }
}

// 8 neighbor offsets (constant to avoid tuple creation)
const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

// ========== Game State ==========
#[derive(Default)]
struct GameOfLife {
    live: HashSet<Position>,
    width: i32,
    height: i32,
    cell: i32,
    generation: u64,
    wrap_world: bool,
    show_grid: bool,
    theme: ColorTheme,
}

impl GameOfLife {
    fn new(width: i32, height: i32, cell_size: i32) -> Self {
        Self {
            live: HashSet::new(),
            width,
            height,
            cell: cell_size,
            generation: 0,
            wrap_world: false,
            show_grid: true,
            theme: ColorTheme::Classic,
        }
    }

    #[inline] fn in_bounds(&self, x: i32, y: i32) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    #[inline] fn wrap(&self, x: i32, y: i32) -> Position {
        let mut nx = x % self.width;
        let mut ny = y % self.height;
        if nx < 0 { nx += self.width; }
        if ny < 0 { ny += self.height; }
        Position(nx, ny)
    }

    fn add_cell(&mut self, x: i32, y: i32) {
        let p = if self.wrap_world { self.wrap(x, y) } else { Position(x, y) };
        if self.wrap_world || self.in_bounds(p.0, p.1) {
            self.live.insert(p);
        }
    }

    fn toggle_cell(&mut self, x: i32, y: i32) {
        let p = if self.wrap_world { self.wrap(x, y) } else { Position(x, y) };
        if !(self.wrap_world || self.in_bounds(p.0, p.1)) { return; }
        if !self.live.remove(&p) { self.live.insert(p); }
    }

    fn clear(&mut self) {
        self.live.clear();
        self.generation = 0;
    }

    fn random_fill(&mut self, density: f32) {
        for y in 0..self.height {
            for x in 0..self.width {
                if gen_range(0.0, 1.0) < density {
                    self.add_cell(x, y);
                }
            }
        }
    }

    /// Next generation: count neighbors with HashMap once -> apply rules
    fn next_generation(&mut self) {
        let mut counts: HashMap<Position, u8> = HashMap::with_capacity(self.live.len() * 8 + 8);

        // Count neighbors of all live cells (distribute to 8 directions)
        for &cell in &self.live {
            for (dx, dy) in NEIGHBOR_OFFSETS {
                let p = if self.wrap_world {
                    self.wrap(cell.0 + dx, cell.1 + dy)
                } else {
                    cell.offset(dx, dy)
                };
                if self.wrap_world || self.in_bounds(p.0, p.1) {
                    *counts.entry(p).or_insert(0) += 1;
                }
            }
        }

        // Create new set from count: 3 -> birth, 2 && alive -> survive
        let mut next = HashSet::with_capacity(self.live.len());
        for (pos, n) in counts {
            let alive = self.live.contains(&pos);
            if n == 3 || (alive && n == 2) {
                next.insert(pos);
            }
        }

        self.live = next;
        self.generation += 1;
    }

    // ---------- Rendering ----------
    fn draw(&self) {
        let colors = self.theme.colors();
        clear_background(colors.background);

        // cells
        for &Position(x, y) in &self.live {
            draw_rectangle(
                (x * self.cell) as f32,
                (y * self.cell) as f32,
                self.cell as f32,
                self.cell as f32,
                colors.cell,
            );
        }

        // grid
        if self.show_grid {
            for x in 0..=self.width {
                draw_line(
                    (x * self.cell) as f32, 0.0,
                    (x * self.cell) as f32, (self.height * self.cell) as f32,
                    1.0, colors.grid,
                );
            }
            for y in 0..=self.height {
                draw_line(
                    0.0, (y * self.cell) as f32,
                    (self.width * self.cell) as f32, (y * self.cell) as f32,
                    1.0, colors.grid,
                );
            }
        }

        // border
        draw_rectangle_lines(
            0.0, 0.0,
            (self.width * self.cell) as f32,
            (self.height * self.cell) as f32,
            3.0, colors.border,
        );
    }

    fn draw_hud(&self, paused: bool, speed: f32) {
        let colors = self.theme.colors();
        let info = format!(
            "Gen:{} | FPS:{:.0} | {} | speed:{:.1} gen/s | grid:{} | wrap:{} | Theme:{}",
            self.generation, get_fps() as f32,
            if paused { "PAUSED" } else { "RUN" },
            speed,
            on_off(self.show_grid),
            on_off(self.wrap_world),
            self.theme.name(),
        );
        draw_text(&info, 10.0, 22.0, 22.0, colors.text);

        let help = "Space: Pause | N: Step | -/=: Speed | R: Random | C: Clear | G: Grid | W: Wrap | T: Theme | Esc: Menu | LMB: Draw/Erase";
        draw_text(help, 10.0, 46.0, 18.0, colors.text_secondary);
    }

    // ---------- Patterns ----------
    fn seed(&mut self, pattern: usize) {
        let gw = self.width;
        let gh = self.height;
        match pattern {
            0 => self.add_glider(gw / 2, gh / 2),
            1 => self.random_fill(RANDOM_DENSITY),
            2 => self.add_block(10, 10),
            3 => self.add_blinker(gw / 2 - 1, gh / 2),
            4 => self.add_beacon(10, 10),
            5 => self.add_rpentomino(gw / 2, gh / 2),
            6 => self.add_acorn(10, 10),
            7 => self.add_diehard(10, 10),
            8 => self.add_gosper_gun(1, 1),
            9 => self.add_pentadecathlon(gw / 2 - 4, gh / 2),
            _ => {}
        }
    }

    fn add_glider(&mut self, x: i32, y: i32) {
        for (dx, dy) in [(1,0),(2,1),(0,2),(1,2),(2,2)] { self.add_cell(x+dx, y+dy); }
    }
    fn add_block(&mut self, x: i32, y: i32) {
        for dx in 0..2 { for dy in 0..2 { self.add_cell(x+dx, y+dy); } }
    }
    fn add_blinker(&mut self, x: i32, y: i32) {
        for dx in 0..3 { self.add_cell(x+dx, y); }
    }
    fn add_beacon(&mut self, x: i32, y: i32) {
        for (dx,dy) in [(0,0),(1,0),(0,1),(2,3),(3,2),(3,3)] { self.add_cell(x+dx, y+dy); }
    }
    fn add_rpentomino(&mut self, x: i32, y: i32) {
        for (dx,dy) in [(1,0),(2,0),(0,1),(1,1),(1,2)] { self.add_cell(x+dx, y+dy); }
    }
    fn add_acorn(&mut self, x: i32, y: i32) {
        for (dx,dy) in [(1,0),(3,1),(0,2),(1,2),(4,2),(5,2),(6,2)] { self.add_cell(x+dx, y+dy); }
    }
    fn add_diehard(&mut self, x: i32, y: i32) {
        for (dx,dy) in [(6,0),(0,1),(1,1),(1,2),(5,2),(6,2),(7,2)] { self.add_cell(x+dx, y+dy); }
    }
    fn add_gosper_gun(&mut self, x: i32, y: i32) {
        let pts = [
            (24,0),(22,1),(24,1),(12,2),(13,2),(20,2),(21,2),(34,2),(35,2),
            (11,3),(15,3),(20,3),(21,3),(34,3),(35,3),(0,4),(1,4),(10,4),(16,4),
            (20,4),(21,4),(0,5),(1,5),(10,5),(14,5),(16,5),(17,5),(22,5),(24,5),
            (10,6),(16,6),(24,6),(11,7),(15,7),(12,8),(13,8),
        ];
        for (dx,dy) in pts { self.add_cell(x+dx, y+dy); }
    }
    fn add_pentadecathlon(&mut self, x: i32, y: i32) {
        for (dx,dy) in [(0,0),(1,0),(2,0),(3,0),(1,-1),(1,1),(4,-1),(4,1),(5,0),(6,0),(7,0),(8,0)] {
            self.add_cell(x+dx, y+dy);
        }
    }
    
    // ---------- Theme Management ----------
    fn cycle_theme(&mut self) {
        self.theme = match self.theme {
            ColorTheme::Classic => ColorTheme::Dark,
            ColorTheme::Dark => ColorTheme::Pastel,
            ColorTheme::Pastel => ColorTheme::Neon,
            ColorTheme::Neon => ColorTheme::Classic,
        };
    }
}

// ========== Menus ==========
async fn choose_resolution(sizes: &[(i32, i32)]) -> usize {
    let mut selected = 1usize;
    loop {
        clear_background(DARKGRAY);
        draw_text("Select screen size:", 20.0, 50.0, 30.0, WHITE);
        for (i, (w, h)) in sizes.iter().enumerate() {
            let marker = if i == selected { ">" } else { " " };
            draw_text(&format!("{} {}x{}", marker, w, h), 40.0, 100.0 + i as f32 * 30.0, 25.0, WHITE);
        }
        draw_text("Enter to confirm", 20.0, 260.0, 25.0, GREEN);

        if is_key_pressed(KeyCode::Up) { selected = (selected + sizes.len() - 1) % sizes.len(); }
        if is_key_pressed(KeyCode::Down) { selected = (selected + 1) % sizes.len(); }
        if is_key_pressed(KeyCode::Enter) { break; }
        next_frame().await;
    }
    next_frame().await;
    selected
}

async fn choose_pattern() -> Option<usize> {
    let names = [
        "Glider","Random","Block","Blinker","Beacon",
        "R-pentomino","Acorn","Diehard","Gosper Gun","Pentadecathlon",
    ];
    let mut selected = 0usize;
    loop {
        clear_background(DARKBLUE);
        draw_text("Select pattern:", 20.0, 50.0, 30.0, WHITE);
        for (i, name) in names.iter().enumerate() {
            let marker = if i == selected { ">" } else { " " };
            draw_text(&format!("{} {}", marker, name), 40.0, 100.0 + i as f32 * 30.0, 25.0, WHITE);
        }
        draw_text("Enter to start | Esc to go back", 20.0, 420.0, 25.0, GREEN);

        if is_key_pressed(KeyCode::Up) { selected = (selected + names.len() - 1) % names.len(); }
        if is_key_pressed(KeyCode::Down) { selected = (selected + 1) % names.len(); }
        if is_key_pressed(KeyCode::Enter) { break Some(selected); }
        if is_key_pressed(KeyCode::Escape) { break None; }
        next_frame().await;
    }
}

// ========== Simulation Loop ==========
async fn run_simulation(screen_w: i32, screen_h: i32, pattern: usize) {
    request_new_screen_size(screen_w as f32, screen_h as f32);

    let grid_w = screen_w / CELL_SIZE;
    let grid_h = screen_h / CELL_SIZE;
    let mut game = GameOfLife::new(grid_w, grid_h, CELL_SIZE);
    game.seed(pattern);

    let mut paused = false;
    let mut speed: f32 = SPEED_INIT;
    let mut acc = 0.0f32;

    loop {
        let dt = get_frame_time();
        acc += dt;

        // ---- input ----
        if is_key_pressed(KeyCode::Space) { paused = !paused; }
        if is_key_pressed(KeyCode::N) && paused { game.next_generation(); }
        if is_key_pressed(KeyCode::Minus) { speed = (speed - 1.0).max(SPEED_MIN); }
        if is_key_pressed(KeyCode::Equal) { speed = (speed + 1.0).min(SPEED_MAX); }
        if is_key_pressed(KeyCode::G) { game.show_grid = !game.show_grid; }
        if is_key_pressed(KeyCode::W) { game.wrap_world = !game.wrap_world; }
        if is_key_pressed(KeyCode::T) { game.cycle_theme(); }
        if is_key_pressed(KeyCode::C) { game.clear(); }
        if is_key_pressed(KeyCode::R) { game.clear(); game.random_fill(RANDOM_DENSITY); }
        if is_key_pressed(KeyCode::Escape) { break; }

        // draw/erase with mouse
        if is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let gx = (mx / game.cell as f32) as i32;
            let gy = (my / game.cell as f32) as i32;
            game.toggle_cell(gx, gy);
        }

        // ---- update ----
        if !paused {
            let step = 1.0 / speed;
            while acc >= step {
                game.next_generation();
                acc -= step;
            }
        }

        // ---- render ----
        game.draw();
        game.draw_hud(paused, speed);
        next_frame().await;
    }
}

// ========== Entry ==========
#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let sizes = [(640, 480), (800, 600), (1024, 768), (1280, 720), (1920, 1080)];
    loop {
        let idx = choose_resolution(&sizes).await;
        let (w, h) = sizes[idx];
        if let Some(pat) = choose_pattern().await {
            run_simulation(w, h, pat).await; // Esc to go back
        }
    }
}

// ========== Utils ==========
#[inline]
fn on_off(b: bool) -> &'static str { if b { "on" } else { "off" } }
