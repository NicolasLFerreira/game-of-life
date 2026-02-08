use crate::{logical_step, Grid, CELL_SIZE_PX};

struct SimulationState {
    cells: Grid,
    is_running: bool,
    step_once: bool,
}

pub struct App {
    grid_pan: egui::Vec2,
    show_grid: bool,
    sim_state: SimulationState,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            grid_pan: egui::Vec2::default(),
            show_grid: true,
            sim_state: SimulationState {
                cells: Grid::new(),
                step_once: false,
                is_running: false,
            },
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::containers::menu::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        println!("New clicked");
                    }
                    if ui.button("Open").clicked() {
                        println!("Open clicked");
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        println!("Undo");
                    }
                });
            });
        });
        egui::SidePanel::left("stats").show(ctx, |ui| {
            ui.heading("Simulation Statistics");
        });
        egui::SidePanel::right("tools").show(ctx, |ui| {
            ui.heading("Grid tools");
            ui.checkbox(&mut self.show_grid, "Show Grid (laggy)");
            ui.heading("Simulation tools");
            ui.checkbox(&mut self.sim_state.is_running, "Run");
            if ui.button("Step").clicked() {
                self.sim_state.step_once = true;
            }
            if ui.button("Clear").clicked() {
                self.sim_state.cells = Grid::new();
                self.grid_pan = egui::Vec2::default();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

            // display geometry values
            let viewport = response.rect;
            let anchor = viewport.min;
            let size = viewport.size();
            let center = anchor + size * 0.5;

            // panning
            if response.dragged() {
                let input = response.ctx.input(|i| i.clone());
                if input.pointer.button_down(egui::PointerButton::Middle) {
                    self.grid_pan += response.drag_delta();
                }
            }

            // paint cells
            if response.clicked() || response.dragged() {
                let input = response.ctx.input(|i| i.clone());

                let Some(click) = response.interact_pointer_pos() else {
                    return;
                };

                let x = ((click.x - center.x - self.grid_pan.x) / CELL_SIZE_PX).floor() as i32;
                let y = ((click.y - center.y - self.grid_pan.y) / CELL_SIZE_PX).floor() as i32;

                if input.pointer.button_down(egui::PointerButton::Primary) {
                    self.sim_state.cells.insert((x, y));
                } else if input.pointer.button_down(egui::PointerButton::Secondary) {
                    self.sim_state.cells.remove(&(x, y));
                }
            }

            // EXTREMELY unoptimized, however, it works, so I'll leave it here for now
            if self.show_grid {
                let start_x = anchor.x as i32;
                let start_y = anchor.y as i32;
                let end_x = size.x as i32;
                let end_y = size.y as i32;

                for x in start_x..end_x {
                    for y in start_y..end_y {
                        paint_cell(
                            &painter,
                            center,
                            x - center.x as i32,
                            y - center.y as i32,
                            self.grid_pan,
                            if (x + y) % 2 == 0 {
                                egui::Color32::LIGHT_GRAY
                            } else {
                                egui::Color32::GRAY
                            },
                        );
                    }
                }
            } else {
                // when no grid just show white
                painter.rect_filled(viewport, 0.0, egui::Color32::WHITE);
            }

            for cell in self.sim_state.cells.iter() {
                paint_cell(
                    &painter,
                    center,
                    cell.0,
                    cell.1,
                    self.grid_pan,
                    egui::Color32::BLACK,
                );
            }
        });

        // Sim run, uncapped rn
        if self.sim_state.is_running || self.sim_state.step_once {
            self.sim_state.step_once = false;
            logical_step(&mut self.sim_state.cells);
            ctx.request_repaint()
        }
    }
}

#[inline(always)]
fn paint_cell(
    painter: &egui::Painter,
    origin: egui::Pos2,
    wx: i32,
    wy: i32,
    pan: egui::Vec2,
    color: egui::Color32,
) {
    let pos = egui::pos2(
        origin.x + wx as f32 * CELL_SIZE_PX + pan.x,
        origin.y + wy as f32 * CELL_SIZE_PX + pan.y,
    );

    let rect = egui::Rect::from_min_size(pos, egui::vec2(CELL_SIZE_PX, CELL_SIZE_PX));

    painter.rect_filled(rect, 0.0, color);
}