use crate::message::{FileOperation, Message};
use iced::border::Radius;
use iced::widget::{button, text};
use iced::{alignment, Border, Color, Element, Length};
use iced_aw::{quad, widget::InnerBounds};

fn base_menu_button<'a>(
    content: impl Into<Element<'a, Message>>,
    msg: Message,
) -> button::Button<'a, Message> {
    button(content)
        .padding([4, 8])
        .style(button::primary)
        .on_press(msg)
}

fn labeled_button(
    label: &str,
    msg: Message,
) -> button::Button<Message, iced::Theme, iced::Renderer> {
    base_menu_button(text(label).align_y(alignment::Vertical::Center), msg)
}

fn menu_sub_button(
    label: &str,
    msg: Message,
) -> button::Button<Message, iced::Theme, iced::Renderer> {
    labeled_button(label, msg).width(Length::Fill)
}

pub fn menu_sub_button_file_open<'a>() -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    menu_sub_button("Open", Message::FileOperation(FileOperation::Open))
}

pub fn menu_sub_button_file_save<'a>() -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    menu_sub_button("Save", Message::FileOperation(FileOperation::Save))
}

pub fn menu_sub_button_file_save_as<'a>() -> button::Button<'a, Message, iced::Theme, iced::Renderer>
{
    menu_sub_button("Save As", Message::FileOperation(FileOperation::SaveAs))
}

pub fn menu_main_button(label: &str) -> button::Button<Message, iced::Theme, iced::Renderer> {
    labeled_button(label, Message::FileOperation(FileOperation::Open)).width(Length::Shrink)
}

pub fn separator() -> quad::Quad {
    quad::Quad {
        quad_color: Color::from([0.5; 3]).into(),
        quad_border: Border {
            radius: Radius::new(4.0),
            ..Default::default()
        },
        inner_bounds: InnerBounds::Ratio(0.98, 0.2),
        height: Length::Fixed(20.0),
        ..Default::default()
    }
}
