use macroquad::prelude::*;

use crate::config::SCREEN_SIZES;
use crate::patterns::{get_pattern_names, get_pattern_by_index};

/// Display the resolution selection menu and return the selected index
pub async fn choose_resolution() -> usize {
    let mut selected = 1usize;
    loop {
        clear_background(DARKGRAY);
        draw_text("Select screen size:", 20.0, 50.0, 30.0, WHITE);
        for (i, (w, h)) in SCREEN_SIZES.iter().enumerate() {
            let marker = if i == selected { ">" } else { " " };
            draw_text(&format!("{} {}x{}", marker, w, h), 40.0, 100.0 + i as f32 * 30.0, 25.0, WHITE);
        }
        draw_text("Enter to confirm", 20.0, 260.0, 25.0, GREEN);

        if is_key_pressed(KeyCode::Up) { selected = (selected + SCREEN_SIZES.len() - 1) % SCREEN_SIZES.len(); }
        if is_key_pressed(KeyCode::Down) { selected = (selected + 1) % SCREEN_SIZES.len(); }
        if is_key_pressed(KeyCode::Enter) { break; }
        next_frame().await;
    }
    next_frame().await;
    selected
}

/// Display the pattern selection menu and return the selected pattern index
pub async fn choose_pattern() -> Option<usize> {
    let names = get_pattern_names();
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

/// Run the main game simulation loop
pub async fn run_simulation(screen_w: i32, screen_h: i32, pattern_index: usize) {
    use crate::config::{CELL_SIZE, SPEED_INIT, SPEED_MAX, SPEED_MIN};
    use crate::game::GameOfLife;
    
    request_new_screen_size(screen_w as f32, screen_h as f32);

    let grid_w = screen_w / CELL_SIZE;
    let grid_h = screen_h / CELL_SIZE;
    let mut game = GameOfLife::new(grid_w, grid_h, CELL_SIZE);
    
    let pattern = get_pattern_by_index(pattern_index);
    let position = (
        if pattern_index == 1 { None } else { Some(grid_w / 2) }, // Center for non-random patterns
        if pattern_index == 1 { None } else { Some(grid_h / 2) }
    );
    
    match position {
        (Some(x), Some(y)) => game.apply_pattern(pattern.as_ref(), x, y),
        _ => game.apply_pattern(pattern.as_ref(), 0, 0), // For random pattern
    }

    let mut paused = false;
    let mut speed: f32 = SPEED_INIT;
    let mut acc = 0.0f32;

    loop {
        let dt = get_frame_time();
        acc += dt;

        // Handle input
        if is_key_pressed(KeyCode::Space) { paused = !paused; }
        if is_key_pressed(KeyCode::N) && paused { game.next_generation(); }
        if is_key_pressed(KeyCode::Minus) { speed = (speed - 1.0).max(SPEED_MIN); }
        if is_key_pressed(KeyCode::Equal) { speed = (speed + 1.0).min(SPEED_MAX); }
        if is_key_pressed(KeyCode::G) { game.show_grid = !game.show_grid; }
        if is_key_pressed(KeyCode::W) { game.grid.wrap_world = !game.grid.wrap_world; }
        if is_key_pressed(KeyCode::T) { game.cycle_theme(); }
        if is_key_pressed(KeyCode::C) { game.clear(); }
        if is_key_pressed(KeyCode::R) { game.clear(); game.random_fill(0.2); }
        if is_key_pressed(KeyCode::Escape) { break; }

        // Handle mouse input
        if is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let gx = (mx / game.cell as f32) as i32;
            let gy = (my / game.cell as f32) as i32;
            game.toggle_cell(gx, gy);
        }

        // Update simulation
        if !paused {
            let step = 1.0 / speed;
            while acc >= step {
                game.next_generation();
                acc -= step;
            }
        }

        // Render
        game.draw();
        game.draw_hud(paused, speed);
        next_frame().await;
    }
}