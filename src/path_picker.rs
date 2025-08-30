use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use egui::{
    PopupCloseBehavior, Response, ScrollArea, TextBuffer, Ui, Widget,
    containers::menu::{MenuButton, MenuConfig},
};

/// A widget for picking files.
///
/// It boils down to a text input, next to a button that opens a widget that
/// allows the user to search the local files and directories.
pub struct PathPicker<'input, 'path, S: TextBuffer> {
    /// Buffer into which store the input
    input: &'input mut S,
    /// The path to fall back upon when the user inputs an invalid path
    default_path: &'path Path,
}

impl<'input, 'path, S: TextBuffer> PathPicker<'input, 'path, S> {
    /// Creates a new PathPicker widget
    ///
    /// ## Args
    ///
    /// - input: the buffer to use for input storage
    /// - default_path: the path to fall back onto when the user inputs an invalid path
    pub fn new<P: AsRef<Path>>(input: &'input mut S, default_path: &'path P) -> Self {
        Self {
            input,
            default_path: default_path.as_ref(),
        }
    }

    /// Internal method that renders the file picker widget
    fn picker_widget(self, ui: &mut Ui) {
        let mut search_path = PathBuf::from(self.input.as_str());
        if !search_path.exists() {
            search_path = self.default_path.to_owned();
        } else if !search_path.is_dir() {
            search_path = search_path.parent().unwrap_or(self.default_path).to_owned();
        }

        search_path = search_path.canonicalize().unwrap();

        if let Some(parent) = search_path.parent() {
            if ui.button("⤴").clicked() {
                self.input.replace_with(parent.to_string_lossy().as_ref());
            }
        }

        if let Ok(iter) = read_dir(search_path) {
            for ent in iter {
                if let Ok(ent) = ent {
                    let path = ent.path();

                    let icon = if path.is_file() { "" } else { "📁" };

                    if ui
                        .button(format!("{} {}", icon, ent.file_name().display()))
                        .clicked()
                    {
                        self.input.replace_with(path.to_string_lossy().as_ref());
                        if path.is_file() {
                            ui.close();
                        }
                    }
                }
            }
        }
    }
}

impl<S: TextBuffer> Widget for PathPicker<'_, '_, S> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(self.input);

            MenuButton::new("📂")
                .config(
                    MenuConfig::default().close_behavior(PopupCloseBehavior::CloseOnClickOutside),
                )
                .ui(ui, |ui| {
                    ui.set_max_height(200.);
                    ui.set_max_width(200.);
                    ScrollArea::vertical().show(ui, |ui| {
                        self.picker_widget(ui);
                    })
                })
        })
        .response
    }
}
