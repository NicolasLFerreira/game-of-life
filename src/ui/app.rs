use crate::persistence::DatabaseConnection;
use crate::types::SimulationFeed;
use crate::types::{ConfigurationChainNode, CellConfiguration};
use std::time::Duration;

const CELL_SIZE_PX: f32 = 16.0;

pub struct App {
    grid_pan: egui::Vec2,
    show_grid: bool,
    sim_feed: SimulationFeed,
    cconf: CellConfiguration,
    input_buffer: String,
    can_conf: ConfigurationChainNode,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>, feed: SimulationFeed) -> Self {
        Self {
            grid_pan: egui::Vec2::default(),
            show_grid: false,
            sim_feed: feed,
            cconf: CellConfiguration::default(),
            input_buffer: String::default(),
            can_conf: ConfigurationChainNode::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let payload = self.sim_feed.take();
        // if let Some(cconf) = &payload.cconf {
        //     self.cconf = cconf.clone();
        // }

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
            ui.label("Enter hash:");
            let response = ui.text_edit_singleline(&mut self.input_buffer);

            if response.lost_focus() {
                if let Ok(parsed) = self.input_buffer.parse::<u128>() {
                    let cconf = DatabaseConnection::open().get(parsed).unwrap();
                    self.can_conf = cconf.clone();
                    self.cconf = CellConfiguration::from_packed(cconf.configuration);
                }
            }
        });
        egui::SidePanel::right("tools").show(ctx, |ui| {
            ui.heading("Grid tools");
            ui.checkbox(&mut self.show_grid, "Show Grid (laggy)");
            ui.heading("Simulation tools");
            if ui.button("Step").clicked() {
                let cconf = DatabaseConnection::open().get(self.can_conf.next_hash).unwrap();
                self.can_conf = cconf.clone();
                self.cconf = CellConfiguration::from_packed(cconf.configuration);
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

            for cell in self.cconf.iter() {
                paint_cell(
                    &painter,
                    center,
                    cell.x,
                    cell.y,
                    self.grid_pan,
                    egui::Color32::BLACK,
                );
            }
        });

        ctx.request_repaint_after(Duration::from_millis(16))
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
