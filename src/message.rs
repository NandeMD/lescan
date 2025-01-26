use iced::widget::{scrollable, text_editor};

#[derive(Debug, Clone)]
pub enum Message {
    T1ContentChanged(text_editor::Action),
    T2ContentChanged(text_editor::Action),
    T3ContentChanged(text_editor::Action),

    Scrolled(scrollable::Viewport),

    TabPressed,
    EnterPressed,
}
