use eframe::egui;
use egui::pos2;
use egui_cable::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions {
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|_| Box::new(MyEguiApp::default())),
    )
    .expect("Failed to start native application");
}

#[derive(Default)]
struct MyEguiApp {}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("My window")
            .default_pos(pos2(20.0, 100.0))
            .show(ctx, |ui| {
                ui.add(Port::new(0));
            });
        egui::Window::new("My window 2")
            .default_pos(pos2(200.0, 20.0))
            .show(ctx, |ui| {
                ui.add(Port::new(1));

                ui.add(Cable::new(0, Plug::to(0).lock(), Plug::to(1)));
                ui.add(Cable::new(1, Plug::to(0), Plug::unplugged().lock()));
            });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::WHITE.to_array()
    }
}
