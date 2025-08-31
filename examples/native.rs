mod app;
use app::ExampleApp;

const TITLE: &'static str = "egui_path_picker_example";

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 100.0])
            .with_title(TITLE),
        ..Default::default()
    };
    eframe::run_native(
        TITLE,
        options,
        Box::new(|_cc| Ok(Box::new(ExampleApp::default()))),
    )
}
