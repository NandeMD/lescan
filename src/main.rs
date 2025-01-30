mod app;
use app::TestApp;

mod balloons_table;
mod footer;
mod message;
mod tinput;
mod utils;

fn main() -> iced::Result {
    iced::application("Test", TestApp::update, TestApp::view)
        .subscription(TestApp::subscription)
        .theme(TestApp::theme)
        .run_with(TestApp::new)
}
