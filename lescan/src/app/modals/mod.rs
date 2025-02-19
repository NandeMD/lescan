use iced::{Element, Color};
use iced::widget::{stack, mouse_area, opaque, center, container};

pub mod settings;

#[derive(Debug, Clone)]
pub enum ModalType {
    Settings
}

pub fn modal_handler<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    modal_type: ModalType,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    match modal_type {
        ModalType::Settings => modal(base, settings::settings_modal(), on_blur),
    }
}

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}