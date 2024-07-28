use bevy::prelude::*;

mod page;
mod plugin;

pub use plugin::BookPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PageState {
    #[default]
    Loading,
    Menu,
    Game,
    Level,
    SettingsAudio,
    SettingsDisplay,
    SettingsControl,
    AboutMain,
    AboutAudio,
    AboutVisual,
    Help,
    Dev,
    Leaderboard,
}

pub const PAGES: [&dyn page::PageBase; 13] = [
    &page::loading::Page,
    &page::menu::Page,
    &page::game::Page,
    &page::level::Page,
    &page::settings::audio::Page,
    &page::settings::display::Page,
    &page::settings::control::Page,
    &page::about::main::Page,
    &page::about::audio::Page,
    &page::about::visual::Page,
    &page::help::Page,
    &page::dev::Page,
    &page::leaderboard::Page,
];
