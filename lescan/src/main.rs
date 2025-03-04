//#![windows_subsystem = "windows"]

use rust_i18n::i18n;

mod app;
use app::LeScan;
mod app_cache;
mod message;
mod settings;
mod utils;

i18n!("locales", fallback = "en");

fn main() -> iced::Result {
    #[cfg(debug_assertions)]
    println!("Starting lescan");

    iced::application("LeScan", LeScan::update, LeScan::view)
        .subscription(LeScan::subscription)
        .theme(LeScan::theme)
        .centered()
        .exit_on_close_request(false)
        .run_with(LeScan::new)
}
