use iced::widget::{center, container, markdown, mouse_area, opaque, stack};
use iced::{Color, Element};

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
    app: &'a crate::TestApp,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    match modal_type {
        ModalType::Settings => modal(base, settings::settings_modal(), on_blur),
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
        let about_msg: &str = r#"
This is an app made by [me](https://github.com/NandeMD), for me, and for you! For the love of Hentai!

This app is made with Rust and Iced, and is a work in progress. I'm still learning Rust and Iced, so please be patient with me.

If you have any suggestions, feedback, or bug reports, please let me know!

You can find the source code for this app on [GitHub](https://github.com/NandeMD/lescan).

Thank you for using this app!

- Version: 0.1.0
        "#;
        let parsed_md = markdown::parse(about_msg);
        ModalMarkdowns {
            about: parsed_md.collect(),
        }
    }
}
