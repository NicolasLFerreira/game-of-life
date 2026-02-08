use crate::types::cell_configuration::CellConfiguration;

pub struct SimulationState {
    pub cell_configuration: CellConfiguration,
    pub is_running: bool,
    pub step_once: bool,
}
