

mod config;
mod themes;
mod grid;
mod game;
mod patterns;
mod ui;

use config::SCREEN_SIZES;
use ui::{choose_resolution, choose_pattern, run_simulation};

/// Main entry point for Conway's Game of Life
#[macroquad::main("Conway's Game of Life")]
async fn main() {
    loop {
        // Get user screen resolution selection
        let idx = choose_resolution().await;
        let (w, h) = SCREEN_SIZES[idx];
        
        // Get user pattern selection
        if let Some(pat) = choose_pattern().await {
            // Start simulation with selected options
            run_simulation(w, h, pat).await;
        }
    }
}