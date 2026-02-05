mod conway;
use conway::*;
use macroquad::prelude::*;

const GRID_WIDTH: usize = 21 * 6;
const GRID_HEIGHT: usize = 9 * 6;
const CELL_SIZE: f32 = 16.0;
const UI_WIDTH: f32 = 200.0;
const GRID_WIDTH_PX: f32 = GRID_WIDTH as f32 * CELL_SIZE;
const GRID_HEIGHT_PX: f32 = GRID_HEIGHT as f32 * CELL_SIZE;
const TICK_DURATION: f32 = 0.1;

type Grid = Vec<bool>;
type Coord = (usize, usize);

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Game of Life"),
        window_width: (CELL_SIZE * GRID_WIDTH as f32 + UI_WIDTH) as i32,
        window_height: CELL_SIZE as i32 * GRID_HEIGHT as i32,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cells: Grid = empty_grid();

    // ui stuff
    let buttons = vec!["Start/Stop (SPACE)", "Clear (C)", "Toggle Grid (G)"];

    // // Seeding
    // let glider: Vec<Vec<bool>> = vec![
    //     vec![false, false, true],
    //     vec![true, false, true],
    //     vec![false, true, true],
    // ];
    // let shape = shape_translate(glider.clone(), 16, 24);
    // seed(&mut cells, shape);
    // let shape = shape_translate(glider.clone(), 20, 28);
    // seed(&mut cells, shape);

    // Simulation
    let mut is_running = false;
    let mut show_grid = false;
    let mut tick_timer = 0.0f32;
    loop {
        // time step
        let dt = get_frame_time();
        tick_timer += dt;

        // input stuff
        let (mx, my) = mouse_position();
        if (mx >= 0.0 && mx < GRID_WIDTH_PX) && (my >= 0.0 && my < GRID_HEIGHT_PX) {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (nmy, nmx) = normalize_mouse(my, mx);
                cells[idx(nmy, nmx)] = true;
            }

            if is_mouse_button_pressed(MouseButton::Right) {
                let (nmy, nmx) = normalize_mouse(my, mx);
                cells[idx(nmy, nmx)] = false;
            }
        }

        if is_key_pressed(KeyCode::Space) {
            is_running = !is_running;
        }

        if is_key_pressed(KeyCode::G) {
            show_grid = !show_grid;
        }

        if is_key_pressed(KeyCode::C) {
            cells = empty_grid();
        }

        // render
        clear_background(WHITE);
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if show_grid {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        if (x + y) % 2 == 0 {
                            Color::new(0.92, 0.92, 0.92, 1.0)
                        } else {
                            Color::new(0.96, 0.96, 0.96, 1.0)
                        },
                    );
                }
                if cells[idx(y, x)] {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        BLACK,
                    );
                }
            }
        }

        // Draw UI panel
        draw_rectangle(
            GRID_WIDTH_PX,
            0.0,
            UI_WIDTH,
            GRID_HEIGHT as f32 * CELL_SIZE,
            Color::new(0.8, 0.8, 0.8, 1.0),
        );

        // Example buttons
        let button_height = 50.0;
        let button_margin = 10.0;

        for (i, &label) in buttons.iter().enumerate() {
            let x = GRID_WIDTH_PX + button_margin;
            let y = i as f32 * (button_height + button_margin) + button_margin;
            let w = UI_WIDTH - 2.0 * button_margin;
            let h = button_height;

            draw_rectangle(x, y, w, h, Color::new(0.6, 0.6, 0.6, 1.0));
            draw_text(label, x + 10.0, y + 30.0, 20.0, BLACK);

            // Detect click
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                if mx >= x && mx <= x + w && my >= y && my <= y + h {
                    match i {
                        0 => is_running = !is_running,
                        1 => cells = empty_grid(),
                        2 => show_grid = !show_grid,
                        _ => {}
                    }
                }
            }
        }

        // actual sim run
        if tick_timer >= TICK_DURATION && is_running {
            simulation(&mut cells);
            tick_timer = 0.0;
        }

        next_frame().await;
    }
}

fn normalize_mouse(my: f32, mx: f32) -> (usize, usize) {
    (
        ((my - (my % CELL_SIZE)) / CELL_SIZE) as usize,
        ((mx - (mx % CELL_SIZE)) / CELL_SIZE) as usize,
    )
}

fn empty_grid() -> Grid {
    vec![false; GRID_WIDTH * GRID_HEIGHT]
}
