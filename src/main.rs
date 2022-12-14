#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

const WINDOW_HEIGHT: f32 = 600.0;
const WINDOW_WIDTH: f32 = 960.0;
const CENTER: (f32, f32) = (
    (WINDOW_WIDTH - WINDOW_WIDTH * 0.60),
    (WINDOW_HEIGHT - WINDOW_HEIGHT * 0.5),
);
const PADDING: f32 = 20.0;

#[derive(Default, PartialEq)]
struct GuiMenu {
    name: String,
    number: u32,
    result: u32,
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
}

impl eframe::App for GuiMenu {
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // for the reset button
        let Self {
            name: _,
            number: _,
            result: _,
            allowed_to_close: _,
            show_confirmation_dialog: _,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.collapsing("Themes", |ui| {
                        egui::widgets::global_dark_light_mode_buttons(ui);
                    });
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    egui::reset_button(ui, self);
                });
            
            });
            ui.add_space(2.0);
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("GUIMENU").size(40.0).strong().color(egui::Color32::from_rgb(171, 39, 79)));
            });
            ui.add_space(8.0);
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            ui.vertical_centered(|ui| {
                let tooltip_text = "This is an email adress";
                ui.hyperlink("wasd.qwertz@email.com")
                    .on_hover_text(tooltip_text);
            });
            ui.add_space(2.0);
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                for i in 1..=60 {
                    ui.label(format!("just some txt {}", i as usize));
                }
            });
            ui.add_space(2.0);
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                ui.label("Right");
            });
            ui.add_space(2.0);
        });

        // CentralPanel must be added after all other panels!
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(2.0);
            ui.heading("THE Menu Application");
            ui.separator();

            ui.vertical_centered(|ui| {
                ui.add_sized([240., 80.], egui::Button::new("Big Button Here"));
                ui.add_space(PADDING + 10.0);

                ui.label(egui::RichText::new("Text can have").color(egui::Color32::from_rgb(110, 255, 110)));
                ui.colored_label(egui::Color32::from_rgb(128, 140, 255), "color"); // Shortcut version
                ui.label("and tooltips.").on_hover_text(
                    "This is a multiline tooltip that demonstrates that you can easily add tooltips to any element.\nThis is the second line.\nThis is the third.",
                );
            });
            ui.separator();

            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Enter name: ");
                    ui.text_edit_singleline(&mut self.name);
                });
                ui.add(
                    egui::Slider::new(&mut self.number, 0..=120).text("number"),
                );
                if ui.button("Calculate").clicked() {
                    self.result = calculate_it(self.number);
                }
                ui.add_space(PADDING);
                ui.label(format!(
                    "Name: '{}'",
                    self.name, 
                ));
                ui.label(egui::RichText::new(format!(
                    "result {}",
                    self.result
                )).size(25.0).italics());
            });
            ui.add_space(2.0);

            if self.show_confirmation_dialog {
                // Show confirmation dialog:
                egui::Window::new("Do you want to quit?")
                    .collapsible(false)
                    .resizable(false)
                    .default_pos(CENTER)
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Cancel").clicked() {
                                self.show_confirmation_dialog = false;
                            }

                            if ui.button("Yes!").clicked() {
                                self.allowed_to_close = true;
                                frame.close();
                            }
                        });
                    });
            }
        });
    }
}

fn calculate_it(num: u32) -> u32 {
    num * num
}

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size =
        Some(egui::Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    eframe::run_native(
        "GUI Menu",
        options,
        Box::new(|_cc| Box::new(GuiMenu::default())),
    );
}
