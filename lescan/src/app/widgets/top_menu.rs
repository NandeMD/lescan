use crate::message::{FileOperation, Message};
use iced::widget::{button, text};
use iced::{alignment, Element, Length};
use rust_i18n::t;

fn base_menu_button<'a>(
    content: impl Into<Element<'a, Message>>,
    msg: Message,
) -> button::Button<'a, Message> {
    button(content)
        .padding([4, 8])
        .style(button::primary)
        .on_press(msg)
}

fn labeled_button<'a>(
    label: impl iced::advanced::text::IntoFragment<'a>,
    msg: Message,
) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    base_menu_button(text(label).align_y(alignment::Vertical::Center), msg)
}

fn menu_sub_button<'a>(
    label: impl iced::advanced::text::IntoFragment<'a>,
    msg: Message,
) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    labeled_button(label, msg).width(Length::Fill)
}

pub fn menu_sub_button_file_new<'a>() -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    menu_sub_button(
        t!("file_menu.new"),
        Message::FileOperation(FileOperation::NewFileDialog),
    )
}

pub fn menu_sub_button_file_open<'a>() -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    menu_sub_button(
        t!("file_menu.open"),
        Message::FileOperation(FileOperation::Open),
    )
}

pub fn menu_sub_button_file_save<'a>() -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    menu_sub_button(
        t!("file_menu.save"),
        Message::FileOperation(FileOperation::SaveFileDialog),
    )
}

pub fn menu_sub_button_file_save_as<'a>() -> button::Button<'a, Message, iced::Theme, iced::Renderer>
{
    menu_sub_button(
        t!("file_menu.save_as"),
        Message::FileOperation(FileOperation::SaveAsFileDialog),
    )
}

pub fn menu_main_button<'a>(
    label: impl iced::advanced::text::IntoFragment<'a>,
) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    labeled_button(label, Message::FileOperation(FileOperation::Open))
        .width(Length::Shrink)
        .on_press_maybe(None)
}
