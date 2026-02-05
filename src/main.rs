mod conway;
use conway::*;
use macroquad::prelude::*;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 50;
const CELL_SIZE: f32 = 16f32;
const TICK_DURATION: f32 = 0.2;

type Grid = Vec<bool>;
type Coord = (usize, usize);

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Game of Life"),
        window_width: CELL_SIZE as i32 * GRID_WIDTH as i32,
        window_height: CELL_SIZE as i32 * GRID_HEIGHT as i32,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cells: Grid = vec![false; GRID_WIDTH * GRID_HEIGHT];

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
        if is_mouse_button_pressed(MouseButton::Left) {
            let nmy = ((my - (my % CELL_SIZE)) / CELL_SIZE) as usize;
            let nmx = ((mx - (mx % CELL_SIZE)) / CELL_SIZE) as usize;

            let index = idx(nmy, nmx);
            let cur = cells[index];
            cells[index] = !cur;
        }

        if is_key_pressed(KeyCode::Space) {
            is_running = !is_running;
        }

        if is_key_pressed(KeyCode::G) {
            show_grid = !show_grid;
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

        // actual sim run
        if tick_timer >= TICK_DURATION && is_running {
            simulation(&mut cells);
            tick_timer = 0.0;
        }

        next_frame().await;
    }
}
