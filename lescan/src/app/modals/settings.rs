use iced::Element;
use iced::widget::{container, column, text};

pub fn settings_modal<'a, Message>() -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    container(column![
        text("Settings"),
        text("dummy setting")
    ])
    .width(300)
    .height(200)
    .padding(10)
    .into()
}