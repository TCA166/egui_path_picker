use egui::CentralPanel;
use egui_path_picker::{DefaultIconProvider, PathPicker};

// if you wish to change which icons are used, be sure to create your own struct
// implementing [IconProvider]

#[derive(Default)]
pub struct ExampleApp {
    // giving path_input some initial value is a good idea
    path_input: String,
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Example path input");
            ui.add(PathPicker::<_, DefaultIconProvider>::new(
                &mut self.path_input,
                &".", // used when the user, for example, clears the input box
            ));
        });
    }
}
