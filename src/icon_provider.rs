/// A simple 'static' trait that provides the [crate::PathPicker] with fitting icons to be used in the GUI
pub trait IconProvider {
    const BACK_ICON: &'static str;
    const FOLDER_ICON: &'static str;
    const FILE_ICON: &'static str;
    const OPEN_ICON: &'static str;
}

/// My [IconProvider] of choice
pub struct DefaultIconProvider;

impl IconProvider for DefaultIconProvider {
    const BACK_ICON: &'static str = "⤴";
    const FOLDER_ICON: &'static str = "📁";
    const FILE_ICON: &'static str = "";
    const OPEN_ICON: &'static str = "📂";
}
