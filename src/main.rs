mod conway;
mod thanatos;
mod ui;
mod types;

use conway::simulation::*;
use eframe::Renderer;
use std::collections::HashSet;
use std::time::Instant;
use ui::app::App;

const CELL_SIZE_PX: f32 = 16.0;
type Coord = (i32, i32);
type Grid = HashSet<Coord>;

fn main() {
    let native_options = eframe::NativeOptions {
        centered: true,
        renderer: Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "Thanatos CGoL",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .unwrap();
}

fn logical_step(mut configuration: &mut Grid) {
    let mut start = Instant::now();

    thanatos::tmfroc::run(&configuration);

    let elapsed = start.elapsed();
    println!("Thanatos: {:?}", elapsed);

    start = Instant::now();

    simulation(&mut configuration);

    let elapsed = start.elapsed();
    println!("Simulation: {:?}", elapsed);
}
