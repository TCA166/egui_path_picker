# egui_path_picker

[![Rust](https://github.com/TCA166/egui_path_picker/actions/workflows/rust.yml/badge.svg)](https://github.com/TCA166/egui_path_picker/actions/workflows/rust.yml)
[![Crates.io Version](https://img.shields.io/crates/v/egui_path_picker)](https://crates.io/crates/egui_path_picker)
[![docs.rs](https://img.shields.io/docsrs/egui_path_picker)](https://docs.rs/egui_path_picker)
[![License](https://img.shields.io/crates/l/egui_path_picker)](LICENSE)

Simple egui widget for picking paths.
It boils down to a text input, next to a button that opens a little widget
which showcases local files and folders. Clicking the entries within the
widget will update the path in the text input, meaning you can either
paste in a path, or find the desired path through the widget.

Originally developed for the needs of my other project
[grapevine](https://github.com/TCA166/grapevine), where I needed
something simple and basic.
