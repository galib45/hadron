use std::fs;

use iced::{padding, widget};

use crate::{models::{Game, Settings}, pages::{add_game, home, settings}, utils};

pub enum Page {
    Home(home::State),
    AddGame(add_game::State),
    // EditGame(edit_game::State),
    Settings(settings::State),
}

#[derive(Clone)]
pub enum Message {
    AddGame(add_game::Message),
    Settings(settings::Message),
    Home(home::Message),
    ToHome,
    ToSettings,
}

pub struct App {
    app_data: AppData,
    current_page: Page,
}


#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppData {
    #[serde(default)]
    games: Vec<Game>,
    #[serde(default)]
    settings: Settings,
}

impl AppData {
    pub fn load() -> Self {
        if let Some(data_dir) = dirs::data_local_dir() {
            let app_data_dir = data_dir.join("hadron");
            if !app_data_dir.exists() {
                fs::create_dir_all(&app_data_dir).unwrap();
            }
            let data_file_path = app_data_dir.join("data.toml");
            let contents = fs::read_to_string(&data_file_path).unwrap();
            let app_data = toml::from_str::<Self>(&contents).unwrap();
            app_data
        } else { Self::default() }
    }

    pub fn save(&self) {
        if let Some(data_dir) = dirs::data_local_dir() {
            let app_data_dir = data_dir.join("hadron");
            if !app_data_dir.exists() {
                fs::create_dir_all(&app_data_dir).unwrap();
            }
            let data_file_path = app_data_dir.join("data.toml");
            fs::write(&data_file_path, toml::to_string_pretty(&self).unwrap()).unwrap();
        }
    } 
}

impl App {
    pub fn new() -> Self {
        let app_data = AppData::load();
        Self {
            app_data: app_data.clone(),
            current_page: Page::Home(home::State::load(&app_data.games)),
        }
    } 

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToHome => { self.current_page = Page::Home(home::State::load(&self.app_data.games)); }
            Message::ToSettings => { 
                self.current_page = Page::Settings(settings::State::load(&self.app_data.settings)); 
            },
            Message::Home(message) => {
                if let Page::Home(state) = &mut self.current_page {
                    let action = state.update(message);
                    match action {
                        home::Action::ToAddGame => {
                            self.current_page = Page::AddGame(add_game::State::default());
                        }
                        home::Action::LaunchGame(index) => {
                            utils::launch_game(&self.app_data.games[index], &self.app_data.settings);
                        },
                        home::Action::EditGame(index) => {
                            self.current_page = Page::AddGame(
                                add_game::State::load(&self.app_data.games[index], index)
                            ) 
                        },
                        home::Action::RemoveGame(index) => {
                            self.app_data.games.remove(index);
                            self.app_data.save();
                            self.current_page = Page::Home(home::State::load(&self.app_data.games));
                        },
                        home::Action::None => {}
                    }
                }
            },
            Message::AddGame(message) => {
                if let Page::AddGame(state) = &mut self.current_page {
                    let action = state.update(message);
                    match action {
                        add_game::Action::New(game) => {
                            self.app_data.games.push(game); 
                            self.app_data.save();
                            self.current_page = Page::Home(home::State::load(&self.app_data.games));
                        },
                        add_game::Action::Edit(index, game) => {
                            self.app_data.games[index] = game;
                            self.app_data.save();
                            self.current_page = Page::Home(home::State::load(&self.app_data.games));
                        },
                        add_game::Action::None => {}
                    }
                }
            },
            Message::Settings(message) => {
                if let Page::Settings(state) = &mut self.current_page {
                    let action = state.update(message);
                    match action {
                        settings::Action::Save(settings) => {
                            self.app_data.settings = settings; 
                            self.app_data.save();
                            self.current_page = Page::Home(home::State::default());
                        }
                        settings::Action::None => {}
                    }
                }
            },
        } 
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let header = widget::container(match &self.current_page {
            Page::Home(_) => home::header(),
            Page::AddGame(_) => add_game::header(),
            Page::Settings(_) => settings::header(),
        }).padding(padding::horizontal(32));

        let body = match &self.current_page {
            Page::Home(state) => state.view().map(move |message| Message::Home(message)),
            Page::AddGame(state) => state.view().map(move |message| Message::AddGame(message)),
            Page::Settings(state) => state.view().map(move |message| Message::Settings(message)),
        };

        let base = widget::container(widget::column![header, body].spacing(24)).padding(padding::vertical(32));

        base.into()
    }

    pub fn title(&self) -> String {
        match &self.current_page {
            Page::Home(_) => "Hadron - Home",
            Page::AddGame(_) => "Hadron - Add a new game",
            Page::Settings(_) => "Hadron - Settings",
        }.into()
    }
}

