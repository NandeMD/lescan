use iced::widget::{column, container, text};
use iced::Element;

pub fn settings_modal<'a, Message>() -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    container(column![text("Settings"), text("dummy setting")])
        .width(300)
        .height(200)
        .padding(10)
        .style(container::rounded_box)
        .into()
}
