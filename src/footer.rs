use crate::message::Message;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{container, text as text_macro, Container},
    Length,
};

pub fn footer<'a>(text: impl text_macro::IntoFragment<'a>) -> Container<'a, Message> {
    container(
        text_macro(text)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Left)
            .align_y(Vertical::Center),
    )
    .style(container::rounded_box)
    .padding(5)
}
