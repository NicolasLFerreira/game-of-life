use crate::types::{
    CellConfiguration, CellCoord, MfracOutcome, MfracTerminationReason, SimulationFeed,
    SimulationPayload,
};
use crate::{conway, mfrac};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Default)]
pub struct SimulationParameters {
    pub feed: SimulationFeed,
    pub max_run_count: u32,
    pub max_generation_count: u32,
    pub run_uncapped: bool,
    pub blocking: bool,
}

pub fn start_simulation(params: SimulationParameters) {
    // Thread configuration
    let handle = {
        let clone = Arc::clone(&params.feed);
        thread::spawn(move || {
            state_machine(
                clone,
                params.max_run_count,
                params.max_generation_count,
                params.run_uncapped,
            )
        })
    };

    if params.blocking {
        handle.join().unwrap();
    }
}

fn state_machine(
    feed: SimulationFeed,
    max_run_count: u32,
    max_generation_count: u32,
    uncapped: bool,
) {
    for i in 0..max_run_count {
        let soup = CellConfiguration::cook_soup(i as u64, 5, 5, 0.4);

        println!("\n#-Starting Run {i}");

        // Runs the simulation and retrieves the MFRAC outcome
        let outcome = simulation_run(Arc::clone(&feed), soup, max_generation_count, uncapped);

        println!("|Run finished.");

        // Processes the outcome for reporting
        match outcome {
            MfracOutcome::Collision(hash) => {
                println!("|Collided with: {}", hash);
            }
            MfracOutcome::Termination(reason) => match reason {
                MfracTerminationReason::LimitExceeded(limit) => {
                    println!("|Limit exceeded: {limit}");
                }
                MfracTerminationReason::StaleLife => {}
                MfracTerminationReason::Oscillator => {}
            },
        }
    }
}

fn simulation_run(
    feed: SimulationFeed,
    soup: Vec<CellCoord>,
    max_generation_count: u32,
    uncapped: bool,
) -> MfracOutcome {
    let mut cconf = CellConfiguration::from_soup(soup);
    for _ in 0..max_generation_count {
        // Run Thanatos on current configuration
        let option = mfrac::run_pipeline(&cconf);
        if let Some(outcome) = option {
            return outcome;
        }

        // Step Conway
        let new_cconf = conway::step(&cconf);

        // Publish results
        cconf = new_cconf;
        feed.store(Arc::new(SimulationPayload::new(Some(cconf.clone()))));

        if !uncapped {
            thread::sleep(Duration::from_millis(100));
        }
    }

    MfracOutcome::Termination(MfracTerminationReason::LimitExceeded(max_generation_count))
}
