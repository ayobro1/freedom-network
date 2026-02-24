mod ui;
mod chat;
mod network;

use iced::Application;

fn main() {
    ui::FreedomBrowser::run(iced::Settings::default()).unwrap();
}