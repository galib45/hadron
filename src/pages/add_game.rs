use std::path::{Path, PathBuf};

use iced::{
    padding::{self, horizontal},
    widget, Alignment, Color, Element, Length,
};

use crate::{app, models::Game, resources, widgets::{buttons::{icon_button, icon_text_button}, layouts::row_space_between}};

pub fn header<'a>() -> iced::Element<'a, app::Message> {
    row_space_between(
        widget::text("Add/Edit Game").size(24).into(), 
        icon_text_button(resources::ARROW_BACK_IOS.clone(), 20, 20, Color::WHITE, "Back")
        .style(|theme, status| widget::button::subtle(theme, status))
        .on_press(app::Message::ToHome)
        .into()
    ).into()
}

#[derive(Clone)]
pub enum Message {
    Save,
    GameNameChanged(String),
    CoverPathChanged(String),
    CoverPathDialogOpen,
    ExePathChanged(String),
    ExePathDialogOpen,
    WineprefixChanged(String),
    WineprefixDialogOpen,
}

pub enum Action {
    New(Game),
    Edit(usize, Game),
    None,
}

#[derive(Default, Clone)]
pub struct State {
    game_name: String,
    cover_path: String,
    exe_path: String,
    wineprefix: String,

    edit_index: Option<usize>,
    // validation state
    show_errors: bool,
    game_name_error: Option<String>,
    cover_path_error: Option<String>,
    exe_path_error: Option<String>,
    wineprefix_error: Option<String>,
}

/* ---------------- Validation helpers ---------------- */

fn validate_game_name(s: &str) -> Option<String> {
    if s.trim().is_empty() {
        Some("Game name is required".into())
    } else {
        None
    }
}

fn validate_file_path(s: &str) -> Option<String> {
    if s.trim().is_empty() {
        Some("Path is required".into())
    } else if !Path::new(s).exists() {
        Some("File does not exist".into())
    } else {
        None
    }
}

fn validate_dir_path(s: &str) -> Option<String> {
    if s.trim().is_empty() {
        Some("Path is required".into())
    } else if !Path::new(s).is_dir() {
        Some("Directory does not exist".into())
    } else {
        None
    }
}

/* ---------------- State logic ---------------- */

impl State {
    pub fn load(game: &Game, index: usize) -> Self {
        Self { 
            game_name: game.name.to_owned(), 
            cover_path: game.cover_path.to_string_lossy().into_owned(), 
            exe_path: game.exe_path.to_string_lossy().into_owned(), 
            wineprefix: game.wine_prefix.to_string_lossy().into_owned(), 
            edit_index: Some(index),
            ..Default::default()
        }
    }

    fn validate(&mut self) -> bool {
        self.game_name_error = validate_game_name(&self.game_name);
        self.cover_path_error = validate_file_path(&self.cover_path);
        self.exe_path_error = validate_file_path(&self.exe_path);
        self.wineprefix_error = validate_dir_path(&self.wineprefix);

        self.game_name_error.is_none()
            && self.cover_path_error.is_none()
            && self.exe_path_error.is_none()
            && self.wineprefix_error.is_none()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Save => {
                self.show_errors = true;

                if !self.validate() {
                    return Action::None;
                }

                let game = Game {
                    name: self.game_name.clone(),
                    cover_path: PathBuf::from(&self.cover_path),
                    exe_path: PathBuf::from(&self.exe_path),
                    wine_prefix: PathBuf::from(&self.wineprefix),
                };
                if let Some(index) = self.edit_index {
                    return Action::Edit(index, game);
                } else {
                    return Action::New(game);
                }
            }

            Message::GameNameChanged(v) => self.game_name = v,
            Message::CoverPathChanged(v) => self.cover_path = v,
            Message::ExePathChanged(v) => self.exe_path = v,
            Message::WineprefixChanged(v) => self.wineprefix = v,

            Message::CoverPathDialogOpen => {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.cover_path = path.to_string_lossy().to_string();
                }
            }

            Message::ExePathDialogOpen => {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.exe_path = path.to_string_lossy().to_string();
                }
            }

            Message::WineprefixDialogOpen => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.wineprefix = path.to_string_lossy().to_string();
                }
            }
        }

        Action::None
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        widget::container(
            widget::column![
                widget::space().height(20),

                field(
                    "Game name",
                    widget::text_input("", &self.game_name)
                        .on_input(Message::GameNameChanged),
                    None,
                    self.show_errors.then_some(&self.game_name_error),
                ),

                field(
                    "Cover path",
                    widget::text_input("", &self.cover_path)
                        .on_input(Message::CoverPathChanged),
                    Some(subtle_icon_button(
                        resources::FILE_OPEN.clone(),
                        20, 20, Color::WHITE,
                        Message::CoverPathDialogOpen,
                    )),
                    self.show_errors.then_some(&self.cover_path_error),
                ),

                field(
                    "Executable path",
                    widget::text_input("", &self.exe_path)
                        .on_input(Message::ExePathChanged),
                    Some(subtle_icon_button(
                        resources::FILE_OPEN.clone(),
                        20, 20, Color::WHITE,
                        Message::ExePathDialogOpen,
                    )),
                    self.show_errors.then_some(&self.exe_path_error),
                ),

                field(
                    "Wineprefix",
                    widget::text_input("", &self.wineprefix)
                        .on_input(Message::WineprefixChanged),
                    Some(subtle_icon_button(
                        resources::FOLDER_OPEN.clone(),
                        20, 20, Color::WHITE,
                        Message::WineprefixDialogOpen,
                    )),
                    self.show_errors.then_some(&self.wineprefix_error),
                ),

                widget::space().height(40),

                widget::row![
                    widget::space().width(Length::Fill),
                    widget::button(widget::text("Save"))
                        .padding(horizontal(32).vertical(6))
                        .on_press(Message::Save),
                    widget::space().width(Length::Fill),
                ]
                .width(Length::Fill)
            ]
            .spacing(16),
        )
        .padding(padding::horizontal(32))
        .into()
    }
}

/* ---------------- UI helpers ---------------- */

fn error_text<'a>(err: &'a Option<String>) -> Element<'a, Message> {
    if let Some(msg) = err {
        widget::text(msg)
            .size(12)
            .color(Color::from_rgb(0.95, 0.35, 0.35))
            .into()
    } else {
        widget::space().height(0).into()
    }
}

fn field<'a>(
    label: &'a str,
    input: widget::TextInput<'a, Message>,
    button: Option<widget::Button<'a, Message>>,
    error: Option<&'a Option<String>>,
) -> widget::Column<'a, Message> {
    widget::column![
        form_row(label, input, button),
        if let Some(err) = error {
            widget::row![
                widget::space().width(140),
                error_text(err)
            ]
        } else {
            widget::row![]
        }
    ]
    .spacing(4)
}

fn subtle_icon_button<'a>(
    svg_handle: impl Into<widget::svg::Handle>,
    width: impl Into<Length>,
    height: impl Into<Length>,
    color: Color,
    on_press: Message,
) -> widget::Button<'a, Message> {
    icon_button(svg_handle, width, height, color)
    .style(|theme, status| widget::button::subtle(theme, status))
    .on_press(on_press)
}

fn form_row<'a>(
    label: &'a str,
    input: widget::TextInput<'a, Message>,
    button: Option<widget::Button<'a, Message>>,
) -> widget::Row<'a, Message> {
    widget::row![
        widget::text(label)
            .width(Length::Fixed(128.0))
            .align_x(widget::text::Alignment::Left),
        input.width(Length::Fill).size(14).line_height(1.5),
        button
            .map(Into::<Element<'_, Message>>::into)
            .unwrap_or_else(|| widget::space().into())
    ]
    .spacing(12)
    .align_y(Alignment::Center)
}

