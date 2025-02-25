//#![windows_subsystem = "windows"]

use rust_i18n::i18n;

mod app;
use app::TestApp;
mod app_cache;
mod message;
mod settings;
mod utils;

i18n!("locales", fallback = "en");

fn main() -> iced::Result {
    #[cfg(debug_assertions)]
    println!("Starting TestApp");

    iced::application("Test", TestApp::update, TestApp::view)
        .subscription(TestApp::subscription)
        .theme(TestApp::theme)
        .centered()
        .exit_on_close_request(false)
        .run_with(TestApp::new)
}
