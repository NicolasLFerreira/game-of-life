mod canonical;
mod conway;

use crate::canonical::canonical_entry_point;
use conway::*;
use macroquad::prelude::*;
use std::collections::HashSet;

// Differentiation viewport from grid dimensions purposedly JustInCase TM
const VIEWPORT_WIDTH: usize = 21 * 6;
const VIEWPORT_HEIGHT: usize = 9 * 6;
const GRID_WIDTH: usize = 21 * 6;
const GRID_HEIGHT: usize = 9 * 6;
const CELL_SIZE_PX: f32 = 16.0;
const GRID_WIDTH_PX: f32 = GRID_WIDTH as f32 * CELL_SIZE_PX;
const GRID_HEIGHT_PX: f32 = GRID_HEIGHT as f32 * CELL_SIZE_PX;
const UI_WIDTH_PX: f32 = 200.0;
const TICK_DURATION: f32 = 0.2;

type Coord = (i32, i32);
type Grid = HashSet<Coord>;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Game of Life"),
        window_width: (GRID_WIDTH_PX + UI_WIDTH_PX) as i32,
        window_height: GRID_HEIGHT_PX as i32,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cells: Grid = HashSet::new();

    // UI stuff
    let buttons = vec!["Start/Stop (SPACE)", "Clear (C)", "Toggle Grid (G)"];

    // camera
    let mut camera_x = 0.0f32;
    let mut camera_y = 0.0f32;
    let mut is_panning = false;
    let mut last_mouse_pos = (0.0f32, 0.0f32);

    // Simulation
    let mut is_running = false;
    let mut show_grid = false;
    let mut step_once = false;
    let mut tick_timer = 0.0f32;
    loop {
        // time step
        let dt = get_frame_time();
        tick_timer += dt;

        // input stuff
        let (mx, my) = mouse_position();
        if (mx >= 0.0 && mx < GRID_WIDTH_PX) && (my >= 0.0 && my < GRID_HEIGHT_PX) {
            if (mx >= 0.0 && mx < GRID_WIDTH_PX) && (my >= 0.0 && my < GRID_HEIGHT_PX) {
                if is_mouse_button_down(MouseButton::Left) && !is_panning {
                    let coord = screen_to_world(mx, my, camera_x, camera_y);
                    if !cells.contains(&coord) {
                        cells.insert(coord);
                    }
                }
                if is_mouse_button_down(MouseButton::Right) && !is_panning {
                    let coord = screen_to_world(mx, my, camera_x, camera_y);
                    cells.remove(&coord);
                }
            }

            if is_mouse_button_pressed(MouseButton::Middle) {
                is_panning = true;
                last_mouse_pos = (mx, my);
            }
            if is_mouse_button_released(MouseButton::Middle) {
                is_panning = false;
            }
            if is_panning {
                camera_x += mx - last_mouse_pos.0;
                camera_y += my - last_mouse_pos.1;
                last_mouse_pos = (mx, my);
            }
        }

        if is_key_pressed(KeyCode::Space) {
            is_running = !is_running;
        }

        if is_key_pressed(KeyCode::G) {
            show_grid = !show_grid;
        }

        if is_key_pressed(KeyCode::C) {
            cells = HashSet::new();
        }

        if is_key_pressed(KeyCode::N) {
            step_once = true;
        }

        // Calculate visible cell range
        let start_x = (-camera_x / CELL_SIZE_PX).floor() as i32;
        let start_y = (-camera_y / CELL_SIZE_PX).floor() as i32;
        let end_x = start_x + VIEWPORT_WIDTH as i32 + 1;
        let end_y = start_y + VIEWPORT_HEIGHT as i32 + 1;

        // render
        clear_background(WHITE);
        for y in start_y..end_y {
            for x in start_x..end_x {
                let screen_pos = world_to_screen(x, y, camera_x, camera_y);
                if show_grid {
                    draw_rectangle(
                        screen_pos.0,
                        screen_pos.1,
                        CELL_SIZE_PX,
                        CELL_SIZE_PX,
                        if (x + y) % 2 == 0 {
                            Color::new(0.92, 0.92, 0.92, 1.0)
                        } else {
                            Color::new(0.96, 0.96, 0.96, 1.0)
                        },
                    );
                }
                if cells.contains(&(x, y)) {
                    draw_rectangle(
                        screen_pos.0,
                        screen_pos.1,
                        CELL_SIZE_PX,
                        CELL_SIZE_PX,
                        BLACK,
                    );
                }
            }
        }

        // UI section
        draw_rectangle(
            GRID_WIDTH_PX,
            0.0,
            UI_WIDTH_PX,
            GRID_HEIGHT_PX,
            Color::new(0.8, 0.8, 0.8, 1.0),
        );

        let button_height = 50.0;
        let button_margin = 10.0;

        for (i, &label) in buttons.iter().enumerate() {
            let x = GRID_WIDTH_PX + button_margin;
            let y = i as f32 * (button_height + button_margin) + button_margin;
            let w = UI_WIDTH_PX - 2.0 * button_margin;
            let h = button_height;

            draw_rectangle(x, y, w, h, Color::new(0.6, 0.6, 0.6, 1.0));
            draw_text(label, x + 10.0, y + 30.0, 20.0, BLACK);

            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                if mx >= x && mx <= x + w && my >= y && my <= y + h {
                    match i {
                        0 => is_running = !is_running,
                        1 => cells = HashSet::new(),
                        2 => show_grid = !show_grid,
                        _ => {}
                    }
                }
            }
        }

        // actual sim run
        if (tick_timer >= TICK_DURATION && is_running) || (step_once && !is_running) {
            // Calculate canonical before stepping simulation
            canonical_entry_point(&cells);
            simulation(&mut cells);
            tick_timer = 0.0;
            step_once = false;
        }

        next_frame().await;
    }
}

fn screen_to_world(screen_x: f32, screen_y: f32, cam_x: f32, cam_y: f32) -> Coord {
    let world_x = (screen_x - cam_x) / CELL_SIZE_PX;
    let world_y = (screen_y - cam_y) / CELL_SIZE_PX;
    (world_x.floor() as i32, world_y.floor() as i32)
}

fn world_to_screen(world_x: i32, world_y: i32, cam_x: f32, cam_y: f32) -> (f32, f32) {
    (
        world_x as f32 * CELL_SIZE_PX + cam_x,
        world_y as f32 * CELL_SIZE_PX + cam_y,
    )
}
