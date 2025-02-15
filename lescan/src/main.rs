#![windows_subsystem = "windows"]

mod app;
use app::TestApp;
mod message;
mod utils;

fn main() -> iced::Result {
    iced::application("Test", TestApp::update, TestApp::view)
        .subscription(TestApp::subscription)
        .theme(TestApp::theme)
        .run_with(TestApp::new)
}
