#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

const WINDOW_HEIGHT: f32 = 320.0;
const WINDOW_WIDTH: f32 = 480.0;
const CENTER: (f32, f32) = (WINDOW_HEIGHT * 0.5, WINDOW_WIDTH * 0.5);
const PADDING: f32 = 20.0;

#[derive(Default)]
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("THE Menu Application");
            ui.separator();

            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                ui.label(egui::RichText::new("Text can have").color(egui::Color32::from_rgb(110, 255, 110)));
                ui.colored_label(egui::Color32::from_rgb(128, 140, 255), "color"); // Shortcut version
                ui.label("and tooltips.").on_hover_text(
                    "This is a multiline tooltip that demonstrates that you can easily add tooltips to any element.\nThis is the second line.\nThis is the third.",
                );
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
                ui.label(format!(
                    "Name: '{}', result {}",
                    self.name, self.result
                ));

                ui.add_space(PADDING);

                for i in 1..=60 {
                    ui.label(format!("just some txt {}", i as usize));
                }
            });

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
    options.initial_window_size = Some(egui::Vec2::new(WINDOW_HEIGHT, WINDOW_WIDTH));
    eframe::run_native(
        "GUI Menu",
        options,
        Box::new(|_cc| Box::new(GuiMenu::default())),
    );
}
