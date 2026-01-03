use iced::widget::svg;
use std::sync::LazyLock;

pub static SVG_ADD_2: LazyLock<svg::Handle> = LazyLock::new(|| {
    svg::Handle::from_memory(include_bytes!("../resources/add_2.svg"))
});

pub static ARROW_BACK_IOS: LazyLock<svg::Handle> = LazyLock::new(|| {
    svg::Handle::from_memory(include_bytes!("../resources/arrow_back_ios.svg"))
});

pub static FOLDER_OPEN: LazyLock<svg::Handle> = LazyLock::new(|| {
    svg::Handle::from_memory(include_bytes!("../resources/folder_open.svg"))
});

pub static FILE_OPEN: LazyLock<svg::Handle> = LazyLock::new(|| {
    svg::Handle::from_memory(include_bytes!("../resources/file_open.svg"))
});

pub static SETTINGS: LazyLock<svg::Handle> = LazyLock::new(|| {
    svg::Handle::from_memory(include_bytes!("../resources/settings.svg"))
});
