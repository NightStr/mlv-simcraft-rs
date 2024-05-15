use std::env;
use std::path::PathBuf;
use iced::widget::{column, row, text, text_input, Column, text_editor, Space, progress_bar, button};
use iced::settings::Settings;
use sim::thieving;
use sim::thieving::{ThievingSimConfig, ThievingSimResult};
use iced::{Size, Length, Theme};
use std::sync::{Arc, RwLock};
use native_dialog::FileDialog;
use serde::{Deserialize, Serialize};
use rust_decimal_macros::dec;


const DEFAULT_SAVE_DIR: &str = "saves";
const SAVE_FILE_EXTENSION: &str = "thsave";
const DEFAULT_SAVE_FILE_NAME: &str = "./saves/last_session.thsave";

pub fn main() -> iced::Result {
    if !std::path::Path::new(DEFAULT_SAVE_DIR).exists() {
        std::fs::create_dir(DEFAULT_SAVE_DIR).unwrap();
    }
    let size = Size::new(600.0, 500.0);
    let app = iced::program("Thieving simulation", ThievingGuiState::update, ThievingGuiState::view)
    .settings(Settings {
        window: iced::window::Settings {
            size: size,
            min_size: Some(size),
            ..iced::window::Settings::default()
        },
        default_font: iced::font::Font::with_name("SansSerif"),
        ..Settings::default()
    });
    app.theme(ThievingGuiState::theme).run()
}



#[derive(Debug, Clone)]
enum Message {
    StartSim,
    SimComplete(Option<ThievingSimResult>),
    StopSim,
    HealthRegenerationInterval(String),
    HealthRegenerationAmount(String),
    MaxHealth(String),
    StealInterval(String),
    StealSuccessChance(String),
    MinDamage(String),
    MaxDamage(String),
    MinGold(String),
    MaxGold(String),
    SimsCount(String),
    SaveConfig(Option<PathBuf>),
    LoadConfig(Option<PathBuf>),
    OpenSaveConfigDialog,
    OpenLoadConfigDialog,
}


#[derive(Debug, Serialize, Deserialize)]
struct ThievingConfigState {
    sims_count: u16,
    steal_success_chance: i32,
    config: ThievingSimConfig,
}

impl Default for ThievingConfigState {
    fn default() -> Self {
        match ThievingConfigState::load_from_file(&DEFAULT_SAVE_FILE_NAME.into()) {
            Ok(config) => {
                return config;
            }
            Err(_) => {
                Self {
                    sims_count: 5000,
                    steal_success_chance: 90,
                    config: ThievingSimConfig::new(
                        dec!(8), // in seconds
                        8,
                        720,
                        dec!(2.6), // in seconds
                        0.9,
                        0,
                        157,
                        51,
                        1212,
                    ),
                }
            }
        }
    }
}


impl ThievingConfigState {
    fn save_to_file(&self, path: &PathBuf) {
        let json = serde_json::to_string(&self).unwrap();
        let _ = std::fs::write(path, json);
    }

    fn load_from_file(path: &PathBuf) -> Result<Self, std::io::Error> {
        let json = std::fs::read(path)?;
        let config: ThievingConfigState = serde_json::from_slice(&json)?;
        Ok(config)
    }

    fn clean_message(&self, message: String, allow_dots: bool) -> String {
        let filter = if allow_dots {
            |c: &char| c.is_digit(10) || *c == '.'
        } else {
            |c: &char| c.is_digit(10)
        };
        let filtered_message = message.chars().filter(filter).collect::<String>();
        let last_dot_index = filtered_message.rfind('.').unwrap_or_default();
        filtered_message.chars().enumerate().filter(|(i, c)| *c != '.' || *c == '.' && *i == last_dot_index).map(|(_, c)| c).collect()
    }
    
    fn update(&mut self, message: Message) {
        match message {
            Message::HealthRegenerationInterval(interval) => {
                self.config.health_regeneration_interval = self.clean_message(interval, true).parse().unwrap_or_default();
            }
            Message::HealthRegenerationAmount(interval) => {
                self.config.health_regeneration_amount = self.clean_message(interval, false).parse().unwrap_or_default();
            }
            Message::MaxHealth(max_health) => {
                self.config.max_health = self.clean_message(max_health, false).parse().unwrap_or_default();
            }
            Message::StealInterval(steal_interval) => {
                self.config.steal_interval = self.clean_message(steal_interval, true).parse().unwrap_or_default();
            }
            Message::StealSuccessChance(steal_success_chance) => {
                self.steal_success_chance = self.clean_message(steal_success_chance, false).parse().unwrap_or_default();
                self.config.steal_success_chance = self.steal_success_chance as f32 / 100.0;
            }
            Message::MinDamage(min_damage) => {
                self.config.min_damage = self.clean_message(min_damage, false).parse().unwrap_or_default();
            }
            Message::MaxDamage(max_damage) => {
                self.config.max_damage = self.clean_message(max_damage, false).parse().unwrap_or_default();
            }
            Message::MinGold(min_gold) => {
                self.config.min_gold = self.clean_message(min_gold, false).parse().unwrap_or_default();
            }
            Message::MaxGold(max_gold) => {
                self.config.max_gold = self.clean_message(max_gold, false).parse().unwrap_or_default();
            }
            Message::SimsCount(sims_count) => {
                self.sims_count = self.clean_message(sims_count, false).parse().unwrap_or_default();
            }
            _ => {}
        }
    }


    fn view(&self) -> Column<Message> {
        const TEXT_ALIGNMENT: iced::alignment::Horizontal = iced::alignment::Horizontal::Left;
        let mut column = Column::new();
        type MessageConstructor = fn(String) -> Message;
        for (label, value, message) in [
            ("Regen interval: ", &self.config.health_regeneration_interval.to_string(), Message::HealthRegenerationInterval as MessageConstructor),
            ("Regen amount: ", &self.config.health_regeneration_amount.to_string(), Message::HealthRegenerationAmount as MessageConstructor),
            ("Health: ", &self.config.max_health.to_string(), Message::MaxHealth as MessageConstructor),
            ("Steal interval: ", &self.config.steal_interval.to_string(), Message::StealInterval as MessageConstructor),
            ("Steal chance: ", &self.steal_success_chance.to_string(), Message::StealSuccessChance as MessageConstructor),
            ("Min damage: ", &self.config.min_damage.to_string(), Message::MinDamage as MessageConstructor),
            ("Max damage: ", &self.config.max_damage.to_string(), Message::MaxDamage as MessageConstructor),
            ("Min gold: ", &self.config.min_gold.to_string(), Message::MinGold as MessageConstructor),
            ("Max gold: ", &self.config.max_gold.to_string(), Message::MaxGold as MessageConstructor),
            ("Sims count: ", &self.sims_count.to_string(), Message::SimsCount as MessageConstructor),
        ].iter().cloned() {
            column = column.push(
                row![
                    text(label).width(Length::Fixed(130.0)).horizontal_alignment(TEXT_ALIGNMENT),
                    text_input("", value).on_input(message).width(Length::Fixed(80.0))
                ].align_items(iced::alignment::Alignment::Center).padding(2));
            }
        column
    }
}


struct ThievingGuiState {
    sim_result: text_editor::Content,
    sims: Vec<ThievingSimResult>,
    progress: f32,
    config_stat: ThievingConfigState,
    is_started: Arc<RwLock<bool>>,
    theme: Theme,
}


impl Drop for ThievingGuiState {
    fn drop(&mut self) {
        self.config_stat.save_to_file(&DEFAULT_SAVE_FILE_NAME.into());
    }
}

impl Default for ThievingGuiState {
    fn default() -> Self {
        let sims: Vec<ThievingSimResult> = Vec::new();
        let config_stat = ThievingConfigState::default();
        Self {
            sim_result: text_editor::Content::with_text(
                &thieving::format_thieve_results(&sims)
            ),
            sims,
            progress: 0.0,
            config_stat,
            is_started: Arc::new(RwLock::new(false)),
            theme: Theme::default(),
        }
    }
}

impl ThievingGuiState {
    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match &message {
            Message::StartSim => {
                self.sims.clear();
                self.sim_result = text_editor::Content::with_text(
                    &thieving::format_thieve_results(&self.sims)
                );
                let mut is_started = self.is_started.write().unwrap();
                *is_started = true;
                let config = self.config_stat.config;
                iced::Command::batch((0..self.config_stat.sims_count).map(|id| {
                    let is_started_clone = self.is_started.clone();
                    iced::Command::perform(
                        async move {
                            let is_started= match is_started_clone.read() {
                                Ok(is_started_ref) => *is_started_ref,
                                Err(_) => false,
                            };
                            if is_started == true {
                                println!("Start sim: {}", id);
                                Some(thieving::sim(&config))
                            } else {
                                println!("Simulation stopped");
                                None
                            }
                        },
                        |r| Message::SimComplete(r)
                    )
                }).collect::<Vec<_>>())
            }
            Message::StopSim => {
                let mut is_started = self.is_started.write().unwrap();
                *is_started = false;
                self.progress = 0.0;
                iced::Command::none()
            }
            Message::SimComplete(sim) => {
                if let Some(r) = sim {
                    self.sims.push(*r);
                }
                if *self.is_started.read().unwrap() == true {
                    self.progress = self.sims.len() as f32;
                }
                if self.sims.len() as u16 % (self.config_stat.sims_count / 10) == 0 {
                    self.sim_result = text_editor::Content::with_text(
                        &thieving::format_thieve_results(&self.sims)
                    );
                }
                if self.sims.len() == self.config_stat.sims_count as usize {
                    self.sim_result = text_editor::Content::with_text(
                        &thieving::format_thieve_results(&self.sims)
                    );
                    let mut is_started = self.is_started.write().unwrap();
                    *is_started = false;
                }
                iced::Command::none()
            }

            Message::SaveConfig(path) => {
                if let Some(path) = path {
                    let mut save_path = PathBuf::from(path);
                    save_path.set_extension(SAVE_FILE_EXTENSION);
                    self.config_stat.save_to_file(&save_path);
                }
                iced::Command::none()
            }
            Message::LoadConfig(path) => {
                if let Some(path) = path {
                    match ThievingConfigState::load_from_file(path) {
                        Ok(config) => {
                            self.config_stat = config;
                        }
                        Err(_) => {}
                    }
                }
                iced::Command::none()
            }
            Message::OpenSaveConfigDialog => {
                iced::Command::perform(async {
                    let current_dir = env::current_dir().unwrap();
                    FileDialog::new()
                        .set_location(&current_dir.join(DEFAULT_SAVE_DIR))
                        .set_filename("thieveing_save")
                        .show_save_single_file().unwrap()
                },
                |path| Message::SaveConfig(path)
                )
            }
            Message::OpenLoadConfigDialog => {
                iced::Command::perform(async {
                    FileDialog::new()
                        .set_location(DEFAULT_SAVE_DIR)
                        .add_filter("saves", &[SAVE_FILE_EXTENSION])
                        .show_open_single_file().unwrap()
                },
                |path| Message::LoadConfig(path)
                )
            }
            _ => {
                self.config_stat.update(message);
                println!("{:?}", &self.config_stat.config);
                iced::Command::none()
            }
        }
    }

    fn view(&self) -> Column<Message> {
        let is_started = self.is_started.read().unwrap();
         column![
            row![
                self.config_stat.view(),
                iced::widget::TextEditor::new(&self.sim_result)
                .height(Length::Fill)
            ].height(Length::Shrink).spacing(5),
            Space::with_height(Length::Fixed(10.0)),
            progress_bar(0.0..=self.config_stat.sims_count as f32, self.progress as f32).width(Length::Fill), // Updated arguments
            Space::with_height(Length::Fill),
            row![
                 if *is_started {button("Stop simulation").on_press(Message::StopSim)} else {button("Start simulation").on_press(Message::StartSim)},
                 Space::with_width(Length::Fill),
                 button("Save").on_press(Message::OpenSaveConfigDialog),
                 Space::with_width(Length::Fixed(5.0)),
                 button("Load").on_press(Message::OpenLoadConfigDialog),
             ],
        ].padding(15)
    }
}
