use iced::widget::{column, container, markdown};
use iced::{Alignment, Element, Theme};

pub fn about_modal<'a, Message>(
    app: &'a crate::TestApp,
    on_link_click: impl Fn(markdown::Url) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    container(
        column![markdown(
            &app.modal_markdowns.about,
            markdown::Settings::default(),
            markdown::Style::from_palette(Theme::TokyoNightStorm.palette())
        )
        .map(on_link_click)]
        .align_x(Alignment::Center),
    )
    .into()
}
