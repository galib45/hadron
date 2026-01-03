use iced::widget::text::Wrapping;
use iced::{padding, widget, Alignment, Color, ContentFit, Length};
use iced::widget::image;

use crate::app;
use crate::models::Game;
use crate::resources;
use crate::widgets::buttons::{icon_button, icon_text_button};
use crate::widgets::layouts::row_space_between;

pub fn header<'a>() -> iced::Element<'a, app::Message> {
    row_space_between(
        widget::text("Games").size(24).into(), 
        icon_button(resources::SETTINGS.clone(), 20, 20, Color::WHITE)
        .style(|theme, status| widget::button::subtle(theme, status))
        .on_press(app::Message::ToSettings)
        .into()
    ).into()
}

#[derive(Clone)]
pub enum Message {
    SelectGame(usize),
    LaunchGame(usize),
    EditGame(usize),
    RemoveGame(usize),
    ToAddGame
}

#[derive(Clone)]
pub enum Action {
    LaunchGame(usize),
    EditGame(usize),
    RemoveGame(usize),
    ToAddGame,
    None
}

#[derive(Default, Clone)]
pub struct State {
    pub games: Vec<Game>,
    pub selected_game_index: Option<usize>,
}

impl State {
    pub fn load(games: &Vec<Game>) -> Self {
        Self {
            games: games.to_owned(),
            ..Default::default()
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ToAddGame => Action::ToAddGame,
            Message::SelectGame(index) => {
                self.selected_game_index = Some(index);
                Action::None
            }
            Message::LaunchGame(index) => Action::LaunchGame(index),
            Message::EditGame(index) => Action::EditGame(index),
            Message::RemoveGame(index) => Action::RemoveGame(index),
        }
    }

    pub fn view<'a>(&'a self) -> iced::Element<'a, Message> {
        widget::stack![
            // 1. Scrollable content ONLY
            widget::scrollable(
                widget::container(
                    if self.games.is_empty() {
                        Into::<iced::Element<'_, Message>>::into(
                            widget::text("No Games Added")
                                .size(32).width(696)
                                .color(Color::from_rgba(0.5, 0.5, 0.5, 0.75))
                        )
                    } else {
                        self.games_grid(&self.games)
                            .columns(4).spacing(32)
                            .width(150 * 4 + 32 * 3)
                            .height(Length::Shrink)
                            .into()
                    }
                )
                .padding(padding::horizontal(32))
            )
            .direction(iced::widget::scrollable::Direction::Both {
                horizontal: iced::widget::scrollable::Scrollbar::default(),
                vertical: iced::widget::scrollable::Scrollbar::default(),
            })
            .width(Length::Fill)
            .height(Length::Fill),

            // 2. Floating action button (LAST = on top)
            widget::float(
                icon_text_button(resources::SVG_ADD_2.clone(), 20, 20, Color::WHITE, "Add Game")
                .on_press(Message::ToAddGame)
            ).translate(|original, viewport| {
                iced::Vector::new(
                    viewport.width - original.width - 32.0,
                    viewport.height - original.height - 120.0,
                )
            }),
            
            if let Some(index) = self.selected_game_index {
                widget::float(
                    widget::column![
                        widget::text(&self.games[index].name),
                        widget::row![
                            widget::button(widget::text("Launch"))
                                .style(|theme, status| widget::button::subtle(theme, status))
                                .on_press(Message::LaunchGame(index)),
                            widget::button(widget::text("Edit"))
                                .style(|theme, status| widget::button::subtle(theme, status))
                                .on_press(Message::EditGame(index)),
                            widget::button(widget::text("Remove"))
                                .style(|theme, status| widget::button::subtle(theme, status))
                                .on_press(Message::RemoveGame(index)),
                        ].spacing(8)
                    ].spacing(8)
                ).translate(|original, viewport| {
                    iced::Vector::new(
                        32.0,
                        viewport.height - original.height - 120.0,
                    )
                })
            } else {
                widget::float(widget::space())
            }
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn game_card<'a>(&'a self, index: usize, name: &'a str, cover_path: impl Into<image::Handle>) -> iced::Element<'a, Message> {
        let base = widget::column![
            image(cover_path)
                .width(150).height(200)
                .content_fit(ContentFit::Fill)
                .border_radius(8),
            widget::text(name).wrapping(Wrapping::Word)
        ].width(150).height(280)
        .spacing(8)
        .align_x(Alignment::Center);

        let overlay = widget::container(widget::space())
            .width(150)
            .height(280)
            .style(|_theme| {
                widget::container::Style {
                    background: Some(Color::from_rgba(0.3, 0.6, 1.0, 0.2).into()),
                    border: iced::Border {
                        color: Color::from_rgb(0.3, 0.6, 1.0),
                        width: 2.0,
                        radius: 8.0.into(),
                    },
                    ..Default::default()
                }
            });
        let card = if self.selected_game_index == Some(index) {
            widget::stack![base, overlay]
        } else {
            widget::stack![widget::hover(base, overlay)]
        };
        widget::mouse_area(card)
            .on_press(Message::SelectGame(index))
            .into()
    }

    fn games_grid<'a>(&'a self, games: &'a Vec<Game>) -> widget::Grid<'a, Message> {
        let mut grid = widget::grid::Grid::with_capacity(games.len());
        for (idx, game) in games.iter().enumerate() {
            grid = grid.push(self.game_card(idx, &game.name, game.cover_path.to_str().unwrap()));
        }
        grid
    }
}
