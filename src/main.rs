use macroquad::prelude::*;

mod config;
mod themes;
mod grid;
mod game;
mod patterns;
mod ui;

use config::SCREEN_SIZES;
use ui::{choose_resolution, choose_pattern, run_simulation};

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    loop {
        // Let user select screen resolution
        let idx = choose_resolution().await;
        let (w, h) = SCREEN_SIZES[idx];
        
        // Let user select a pattern
        if let Some(pat) = choose_pattern().await {
            // Run the simulation with the selected pattern
            run_simulation(w, h, pat).await;
        }
    }
}