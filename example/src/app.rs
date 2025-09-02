use egui::CentralPanel;
use egui_path_picker::PathPicker;

#[derive(Default)]
pub struct ExampleApp {
    path_input: String,
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Example path input");
            ui.add(PathPicker::new(&mut self.path_input, &"."));
        });
    }
}
