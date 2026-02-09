use clap::Parser;
use thanatos::startup::{StartupParameters, startup};

#[derive(Parser)]
struct Cli {
    // No GUI
    #[arg(long, default_value_t = false)]
    headless: bool,
    // Run cap
    #[arg(long, default_value_t = 100)]
    max_runs: u32,
    // Generation cap per run
    #[arg(long, default_value_t = 1_000)]
    max_generations: u32,
}

fn main() {
    let cli = Cli::parse();

    // build startup
    let startup_params = StartupParameters {
        max_runs: cli.max_runs,
        max_generations: cli.max_generations,
        run_headless: cli.headless,
    };

    startup(startup_params);
}
