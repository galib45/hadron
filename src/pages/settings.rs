use std::path::{Path, PathBuf};

use iced::{
    padding::{self, horizontal},
    widget, Alignment, Color, Element, Length,
};

use crate::{app, models, resources, widgets::{buttons::{icon_button, icon_text_button}, layouts::row_space_between}};

pub fn header<'a>() -> iced::Element<'a, app::Message> {
    row_space_between(
        widget::text("Settings").size(24).into(), 
        icon_text_button(resources::ARROW_BACK_IOS.clone(), 20, 20, Color::WHITE, "Back")
        .style(|theme, status| widget::button::subtle(theme, status))
        .on_press(app::Message::ToHome)
        .into()
    ).into()
}

#[derive(Clone)]
pub enum Message {
    Save,
    ProtonPathChanged(String),
    UmuPathChanged(String),
    ProtonPathDialogOpen,
    UmuPathDialogOpen,
}

pub enum Action {
    Save(models::Settings),
    None,
}

#[derive(Default, Clone)]
pub struct State {
    proton_path: String,
    umu_path: String,

    // validation state
    show_errors: bool,
    proton_path_error: Option<String>,
    umu_path_error: Option<String>,
}

/* ---------------- Validation helpers ---------------- */

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
    pub fn load(settings: &models::Settings) -> Self {
        Self { 
            proton_path: settings.proton_path.to_string_lossy().into_owned(), 
            umu_path: settings.umu_path.to_string_lossy().into_owned(), 
            ..Default::default() 
        }
    }

    fn validate(&mut self) -> bool {
        self.proton_path_error = validate_dir_path(&self.proton_path);
        self.umu_path_error = validate_dir_path(&self.umu_path);

        self.proton_path_error.is_none()
            && self.umu_path_error.is_none()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Save => {
                self.show_errors = true;

                if !self.validate() {
                    return Action::None;
                }

                return Action::Save(models::Settings {
                    proton_path: PathBuf::from(&self.proton_path),
                    umu_path: PathBuf::from(&self.umu_path),
                });
            }

            Message::ProtonPathChanged(v) => self.proton_path = v,
            Message::UmuPathChanged(v) => self.umu_path = v,

            Message::ProtonPathDialogOpen => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.proton_path = path.to_string_lossy().to_string();
                }
            }

            Message::UmuPathDialogOpen => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.umu_path = path.to_string_lossy().to_string();
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
                    "Proton Path",
                    widget::text_input("", &self.proton_path)
                        .on_input(Message::ProtonPathChanged),
                    Some(subtle_icon_button(
                        resources::FOLDER_OPEN.clone(),
                        20, 20, Color::WHITE,
                        Message::ProtonPathDialogOpen,
                    )),
                    self.show_errors.then_some(&self.proton_path_error),
                ),

                field(
                    "Umu Path",
                    widget::text_input("", &self.umu_path)
                        .on_input(Message::UmuPathChanged),
                    Some(subtle_icon_button(
                        resources::FOLDER_OPEN.clone(),
                        20, 20, Color::WHITE,
                        Message::UmuPathDialogOpen,
                    )),
                    self.show_errors.then_some(&self.umu_path_error),
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

