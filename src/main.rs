use crossbeam::atomic::AtomicCell;
use eframe::Renderer;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use thanatos::conway;
use thanatos::mfroc;
use thanatos::types::cell_configuration::CellConfiguration;
use thanatos::types::cell_coord::CellCoord;
use thanatos::types::simulation_feed::SimulationFeed;
use thanatos::types::simulation_payload::SimulationPayload;
use thanatos::ui::app::App;

fn main() {
    let glider: Vec<CellCoord> = vec![
        CellCoord::new(0, 0),
        CellCoord::new(0, 1),
        CellCoord::new(0, 2),
        CellCoord::new(1, 2),
        CellCoord::new(2, 1),
    ];

    let seed_cells = CellConfiguration::random_configuration(42, 20, 20, 0.3);
    let feed = Arc::new(AtomicCell::new(Arc::new(SimulationPayload::default())));

    // Simulation thread
    {
        let clone = Arc::clone(&feed);
        thread::spawn(move || start_simulation(glider, clone))
    };

    start_ui(Arc::clone(&feed));
}

fn start_simulation(seed_cells: Vec<CellCoord>, feed: SimulationFeed) {
    let mut cconf = CellConfiguration::with_seed_configuration(seed_cells);
    loop {
        // Run Thanatos on current configuration
        mfroc::process_mfroc(&cconf);

        // Step Conway
        let new_cconf = conway::step(&cconf);

        // Publish results
        cconf = new_cconf;
        feed.store(Arc::new(SimulationPayload::new(Some(cconf.clone()))));

        // Currently capping for UI and development
        thread::sleep(Duration::from_millis(2));
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
