use iced::widget::{container, markdown};
use iced::{Element, Pixels, Theme};

pub fn about_modal<'a, Message>(
    app: &'a crate::LeScan,
    on_link_click: impl Fn(markdown::Url) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    container(
        markdown(
            &app.modal_markdowns.about,
            markdown::Settings::default(),
            markdown::Style::from_palette(Theme::TokyoNightStorm.palette()),
        )
        .map(on_link_click),
    )
    .max_width(Pixels::from(400))
    .style(container::rounded_box)
    .padding(10)
    .into()
}
