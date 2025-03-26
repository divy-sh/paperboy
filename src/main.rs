use eframe::egui;

struct MyApp {
    show_subwindow: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            show_subwindow: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|_| {});
        });

        // Side panel with adjustable width
        egui::SidePanel::left("side_panel")
            .resizable(true)
            .min_width(100.0)
            .max_width(200.0)
            .show(ctx, |ui| {
                if ui.button("hello").clicked() {
                    self.show_subwindow = !self.show_subwindow;
                };
                ui.label("This is the sidebar that I want to resize and now it works smoothly");
            });

        if self.show_subwindow {
            egui::Window::new("Subwindow")
                .open(&mut self.show_subwindow)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.label("This is a subwindow!");
                });
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Resizable Sidebar Example",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    );
}
