use iced::widget::{center, container, markdown, mouse_area, opaque, stack};
use iced::{Color, Element};
use rust_i18n::t;
use settings::SettingsTabs;

pub mod about;
pub mod settings;

#[derive(Debug, Clone)]
pub enum ModalType {
    Settings,
    About,
}

pub fn modal_handler<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    modal_type: ModalType,
    on_blur: Message,
    on_link_click: impl Fn(markdown::Url) -> Message + 'a,
    on_settings_tab_click: impl Fn(SettingsTabs) -> Message + 'static,
    app: &'a crate::TestApp,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    match modal_type {
        ModalType::Settings => modal(
            base,
            settings::settings_modal(app, on_settings_tab_click),
            on_blur,
        ),
        ModalType::About => modal(base, about::about_modal(app, on_link_click), on_blur),
    }
}

// Directly copied from
// https://github.com/iced-rs/iced/blob/master/examples/modal/src/main.rs
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

pub struct ModalMarkdowns {
    pub about: Vec<markdown::Item>,
}

impl Default for ModalMarkdowns {
    fn default() -> Self {
        let about_text = t!("about").into_owned();
        let parsed_md = markdown::parse(&about_text);
        ModalMarkdowns {
            about: parsed_md.collect(),
        }
    }
}
