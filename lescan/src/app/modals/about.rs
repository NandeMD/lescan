use iced::widget::{column, container, markdown};
use iced::{Alignment, Element, Theme};

pub fn about_modal<'a, Message>() -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let about_msg = "This is an app made by me, for me, and for you! For the love of Hentai!";
    container(
        column![markdown(
            markdown::parse(about_msg).collect(),
            markdown::Settings::default(),
            markdown::Style::from_palette(Theme::TokyoNightStorm.palette())
        )]
        .align_x(Alignment::Center),
    )
    .style(container::rounded_box)
    .into()
}
