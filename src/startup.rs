use crate::orchestration::{SimulationParameters, start_simulation};
use crate::types::{SimulationFeed, SimulationPayload};
use crate::ui::app::App;
use crossbeam::atomic::AtomicCell;
use eframe::Renderer;
use std::sync::Arc;

pub struct StartupParameters {
    pub max_runs: u32,
    pub max_generations: u32,
    pub run_headless: bool,
}

/// Sets up the different parts of the program
pub fn startup(parameters: StartupParameters) {
    // Simulation state feed setup
    let feed = Arc::new(AtomicCell::new(Arc::new(SimulationPayload::default())));

    start_simulation(SimulationParameters {
        feed: Arc::clone(&feed),
        max_run_count: parameters.max_runs,
        max_generation_count: parameters.max_generations,
        run_uncapped: parameters.run_headless,
        blocking: parameters.run_headless,
    });

    // UI
    if !parameters.run_headless {
        start_ui(Arc::clone(&feed));
    }
}

fn start_ui(feed: SimulationFeed) {
    let native_options = eframe::NativeOptions {
        centered: true,
        renderer: Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "Thanatos",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, feed)))),
    )
    .unwrap();
}
